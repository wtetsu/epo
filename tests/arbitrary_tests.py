import sys
import random
import argparse
import subprocess
from datetime import datetime
from zoneinfo import available_timezones, ZoneInfo
from typing import Callable

EXCEPT = ["Factory", "Africa/Monrovia"]


def main(epo_path: str, repeat: int):
    ok_num = 0
    max_epoch = int(datetime.now().timestamp())

    timezones = [t for t in available_timezones() if t not in EXCEPT]
    runner = make_test_runner(epo_path, timezones, max_epoch)

    for i in range(1, repeat+1):
        r = runner(random.randint(1, 100), random.randint(1, 10))
        if r:
            ok_num += 1
        if i % 10 == 0 or i == repeat:
            print(f"✅ {ok_num: >3}  💀 {i-ok_num: >3}")

    if ok_num == repeat:
        print("All tests passed")
        sys.exit(0)
    else:
        print(f"{repeat-ok_num} tests failed")
        sys.exit(1)


def make_test_runner(epo_path: str, available_timezones: list[str], max_epoch: int) -> Callable[[int, int], bool]:
    def test_runner(epoch_num: int, timezone_num: int):
        epochs = [random.randint(0, max_epoch) for _ in range(epoch_num)]
        timezones = random.sample(available_timezones, timezone_num)
        return run_and_compare(epo_path, epochs, timezones)
    return test_runner


def run_and_compare(epo_path: str, epochs: list[int], timezones: list[str]) -> bool:
    actual = run_epo(epo_path, epochs, timezones)
    expected = generate_cross_check_data(epochs, timezones)

    ok = actual == expected

    if not ok:
        print(" ".join([str(e) for e in epochs] + [t for t in timezones]))
        print("# actual")
        print(actual)
        print("# expected")
        print(expected)

    return actual == expected


def run_epo(epo_path: str, epochs: list[int], timezones: list[str]) -> str:
    command = [epo_path, "-p", *map(str, epochs), *timezones]
    r = subprocess.run(command, capture_output=True, text=True)
    if r.returncode != 0:
        print(r.stderr, file=sys.stderr)
    return r.stdout


def generate_cross_check_data(epochs: list[int], timezones: list[str]) -> str:
    result = ""
    for e in epochs:
        dates = [epoch_to_date_str(e, tz) for tz in timezones]
        result += " ".join([str(e), *dates]) + "\n"
    return result


def epoch_to_date_str(epoch: int, tzname: str) -> str:
    dt = datetime.fromtimestamp(epoch, tz=ZoneInfo(tzname))
    return dt.strftime('%Y-%m-%dT%H:%M:%S%z')


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--epo", default="epo", type=str)
    parser.add_argument("--repeat", default=1000, type=int)
    args = parser.parse_args()

    main(args.epo, args.repeat)
