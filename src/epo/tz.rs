use once_cell::sync::Lazy;

struct Name {
    full_name: String,
    location: String,
}

static DATA: Lazy<Vec<Vec<Name>>> = Lazy::new(|| {
    vec![
        a(),
        b(),
        c(),
        d(),
        e(),
        f(),
        g(),
        h(),
        i(),
        j(),
        k(),
        l(),
        m(),
        n(),
        o(),
        p(),
        q(),
        r(),
        s(),
        t(),
        u(),
        v(),
        w(),
        x(),
        y(),
        z(),
    ]
});

pub fn search(search_name: &str) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();

    let r = search_name.chars().next();

    if let Some(first_letter) = r {
        let c = (first_letter.to_ascii_lowercase() as i32) - ('a' as i32);
        if (0..26).contains(&c) {
            let data = &DATA[c as usize];
            let lower_search_name = &search_name.to_lowercase();
            for name in data {
                if name.location.starts_with(lower_search_name) {
                    found.push(name.full_name.clone());
                }
            }
        }
    }

    found
}

fn new_name(full_tz_name: &str) -> Name {
    let r = full_tz_name.rfind('/');

    if let Some(idx) = r {
        let location = full_tz_name[(idx + 1)..].to_lowercase();
        let full_name = full_tz_name.to_string();
        return Name { full_name, location };
    }

    panic!();
}

// Time Zone Database
// https://www.iana.org/time-zones

fn a() -> Vec<Name> {
    vec![
        new_name("Africa/Abidjan"),
        new_name("Africa/Accra"),
        new_name("Africa/Addis_Ababa"),
        new_name("Africa/Algiers"),
        new_name("Africa/Asmara"),
        new_name("America/Adak"),
        new_name("America/Anchorage"),
        new_name("America/AnguillaSandy Hill"),
        new_name("America/Antigua"),
        new_name("America/Araguaina"),
        new_name("America/Aruba"),
        new_name("America/Asuncion"),
        new_name("America/Atikokan"),
        new_name("Asia/Aden"),
        new_name("Asia/Almaty"),
        new_name("Asia/Amman"),
        new_name("Asia/Anadyr"),
        new_name("Asia/Aqtau"),
        new_name("Asia/Aqtobe"),
        new_name("Asia/Ashgabat"),
        new_name("Asia/Atyrau"),
        new_name("Atlantic/Azores"),
        new_name("Australia/Adelaide"),
        new_name("Europe/Amsterdam"),
        new_name("Europe/Andorra"),
        new_name("Europe/Astrakhan"),
        new_name("Europe/Athens"),
        new_name("Indian/Antananarivo"),
        new_name("Pacific/Apia"),
        new_name("Pacific/Auckland"),
    ]
}

fn b() -> Vec<Name> {
    vec![
        new_name("Africa/Bamako"),
        new_name("Africa/Bangui"),
        new_name("Africa/Banjul"),
        new_name("Africa/Bissau"),
        new_name("Africa/Blantyre"),
        new_name("Africa/Brazzaville"),
        new_name("Africa/Bujumbura"),
        new_name("America/Argentina/Buenos_Aires"),
        new_name("America/Bahia"),
        new_name("America/Bahia_Banderas"),
        new_name("America/Barbados"),
        new_name("America/Belem"),
        new_name("America/Belize"),
        new_name("America/Blanc-Sablon"),
        new_name("America/Boa_Vista"),
        new_name("America/Bogota"),
        new_name("America/Boise"),
        new_name("America/North_Dakota/Beulah"),
        new_name("Asia/Baghdad"),
        new_name("Asia/Bahrain"),
        new_name("Asia/Baku"),
        new_name("Asia/Bangkok"),
        new_name("Asia/Barnaul"),
        new_name("Asia/Beirut"),
        new_name("Asia/Bishkek"),
        new_name("Asia/Brunei"),
        new_name("Atlantic/Bermuda"),
        new_name("Australia/Brisbane"),
        new_name("Australia/Broken_Hill"),
        new_name("Europe/Belgrade"),
        new_name("Europe/Berlin"),
        new_name("Europe/Bratislava"),
        new_name("Europe/Brussels"),
        new_name("Europe/Bucharest"),
        new_name("Europe/Budapest"),
        new_name("Pacific/Bougainville"),
    ]
}

fn c() -> Vec<Name> {
    vec![
        new_name("Africa/Cairo"),
        new_name("Africa/Casablanca"),
        new_name("Africa/Ceuta"),
        new_name("Africa/Conakry"),
        new_name("America/Argentina/Catamarca"),
        new_name("America/Argentina/Cordoba"),
        new_name("America/Cambridge_Bay"),
        new_name("America/Campo_Grande"),
        new_name("America/Cancun"),
        new_name("America/Caracas"),
        new_name("America/Cayenne"),
        new_name("America/Cayman"),
        new_name("America/Chicago"),
        new_name("America/Chihuahua"),
        new_name("America/Costa_Rica"),
        new_name("America/Creston"),
        new_name("America/Cuiaba"),
        new_name("America/Curacao"),
        new_name("America/North_Dakota/Center"),
        new_name("Antarctica/Casey"),
        new_name("Asia/Chita"),
        new_name("Asia/Choibalsan"),
        new_name("Asia/Colombo"),
        new_name("Atlantic/Canary"),
        new_name("Atlantic/Cape_Verde"),
        new_name("Australia/Currie"),
        new_name("Europe/Chisinau"),
        new_name("Europe/Copenhagen"),
        new_name("Indian/Chagos"),
        new_name("Indian/Christmas"),
        new_name("Indian/Cocos"),
        new_name("Indian/Comoro"),
        new_name("Pacific/Chatham"),
        new_name("Pacific/Chuuk"),
    ]
}

fn d() -> Vec<Name> {
    vec![
        new_name("Africa/Dakar"),
        new_name("Africa/Dar_es_Salaam"),
        new_name("Africa/Djibouti"),
        new_name("Africa/Douala"),
        new_name("America/Danmarkshavn"),
        new_name("America/Dawson"),
        new_name("America/Dawson_Creek"),
        new_name("America/Denver"),
        new_name("America/Detroit"),
        new_name("America/Dominica"),
        new_name("Antarctica/Davis"),
        new_name("Antarctica/DumontDUrville"),
        new_name("Asia/Damascus"),
        new_name("Asia/Dhaka"),
        new_name("Asia/Dili"),
        new_name("Asia/Dubai"),
        new_name("Asia/Dushanbe"),
        new_name("Australia/Darwin"),
        new_name("Europe/Dublin"),
    ]
}

fn e() -> Vec<Name> {
    vec![
        new_name("Africa/El_Aaiun"),
        new_name("America/Edmonton"),
        new_name("America/Eirunepe"),
        new_name("America/El_Salvador"),
        new_name("Australia/Eucla"),
        new_name("Pacific/Easter"),
        new_name("Pacific/Efate"),
        new_name("Pacific/Enderbury"),
    ]
}

fn f() -> Vec<Name> {
    vec![
        new_name("Africa/Freetown"),
        new_name("America/Fort_Nelson"),
        new_name("America/Fortaleza"),
        new_name("Asia/Famagusta"),
        new_name("Atlantic/Faroe"),
        new_name("Pacific/Fakaofo"),
        new_name("Pacific/Fiji"),
        new_name("Pacific/Funafuti"),
    ]
}

fn g() -> Vec<Name> {
    vec![
        new_name("Africa/Gaborone"),
        new_name("America/Glace_Bay"),
        new_name("America/Godthab"),
        new_name("America/Goose_Bay"),
        new_name("America/Grand_Turk"),
        new_name("America/Grenada"),
        new_name("America/Guadeloupe"),
        new_name("America/Guatemala"),
        new_name("America/Guayaquil"),
        new_name("America/Guyana"),
        new_name("Asia/Gaza"),
        new_name("Europe/Gibraltar"),
        new_name("Europe/Guernsey"),
        new_name("Pacific/Galapagos"),
        new_name("Pacific/Gambier"),
        new_name("Pacific/Guadalcanal"),
        new_name("Pacific/GuamVillage"),
    ]
}

fn h() -> Vec<Name> {
    vec![
        new_name("Africa/Harare"),
        new_name("America/Halifax"),
        new_name("America/Havana"),
        new_name("America/Hermosillo"),
        new_name("Asia/Hebron"),
        new_name("Asia/Ho_Chi_Minh"),
        new_name("Asia/Hong_Kong"),
        new_name("Asia/Hovd"),
        new_name("Australia/Hobart"),
        new_name("Europe/Helsinki"),
        new_name("Pacific/Honolulu"),
    ]
}

fn i() -> Vec<Name> {
    vec![
        new_name("America/Indiana/Indianapolis"),
        new_name("America/Inuvik"),
        new_name("America/Iqaluit"),
        new_name("Asia/Irkutsk"),
        new_name("Europe/Isle_of_Man"),
        new_name("Europe/Istanbul"),
    ]
}

fn j() -> Vec<Name> {
    vec![
        new_name("Africa/Johannesburg"),
        new_name("Africa/Juba"),
        new_name("America/Argentina/Jujuy"),
        new_name("America/Jamaica"),
        new_name("America/Juneau"),
        new_name("Asia/Jakarta"),
        new_name("Asia/Jayapura"),
        new_name("Asia/Jerusalem"),
        new_name("Europe/Jersey"),
    ]
}

fn k() -> Vec<Name> {
    vec![
        new_name("Africa/Kampala"),
        new_name("Africa/Khartoum"),
        new_name("Africa/Kigali"),
        new_name("Africa/Kinshasa"),
        new_name("America/Indiana/Knox"),
        new_name("America/Kralendijk"),
        new_name("Asia/Kabul"),
        new_name("Asia/Kamchatka"),
        new_name("Asia/Karachi"),
        new_name("Asia/Kathmandu"),
        new_name("Asia/Khandyga"),
        new_name("Asia/Kolkata"),
        new_name("Asia/Krasnoyarsk"),
        new_name("Asia/Kuala_Lumpur"),
        new_name("Asia/Kuching"),
        new_name("Asia/Kuwait"),
        new_name("Europe/Kaliningrad"),
        new_name("Europe/Kiev"),
        new_name("Europe/Kirov"),
        new_name("Indian/Kerguelen"),
        new_name("Pacific/Kiritimati"),
        new_name("Pacific/Kosrae"),
        new_name("Pacific/Kwajalein"),
    ]
}

fn l() -> Vec<Name> {
    vec![
        new_name("Africa/Lagos"),
        new_name("Africa/Libreville"),
        new_name("Africa/Lome"),
        new_name("Africa/Luanda"),
        new_name("Africa/Lubumbashi"),
        new_name("Africa/Lusaka"),
        new_name("America/Argentina/La_Rioja"),
        new_name("America/Kentucky/Louisville"),
        new_name("America/La_Paz"),
        new_name("America/Lima"),
        new_name("America/Los_Angeles"),
        new_name("America/Lower_Princes"),
        new_name("Arctic/Longyearbyen"),
        new_name("Australia/Lindeman"),
        new_name("Australia/Lord_Howe"),
        new_name("Europe/Lisbon"),
        new_name("Europe/Ljubljana"),
        new_name("Europe/London"),
        new_name("Europe/Luxembourg"),
    ]
}

fn m() -> Vec<Name> {
    vec![
        new_name("Africa/Malabo"),
        new_name("Africa/Maputo"),
        new_name("Africa/Maseru"),
        new_name("Africa/Mbabane"),
        new_name("Africa/Mogadishu"),
        new_name("Africa/Monrovia"),
        new_name("America/Argentina/Mendoza"),
        new_name("America/Indiana/Marengo"),
        new_name("America/Kentucky/Monticello"),
        new_name("America/Maceio"),
        new_name("America/Managua"),
        new_name("America/Manaus"),
        new_name("America/Marigot"),
        new_name("America/Martinique"),
        new_name("America/Matamoros"),
        new_name("America/Mazatlan"),
        new_name("America/Menominee"),
        new_name("America/Merida"),
        new_name("America/Metlakatla"),
        new_name("America/Mexico_City"),
        new_name("America/Miquelon"),
        new_name("America/Moncton"),
        new_name("America/Monterrey"),
        new_name("America/Montevideo"),
        new_name("America/Montserrat"),
        new_name("Antarctica/Macquarie"),
        new_name("Antarctica/Mawson"),
        new_name("Antarctica/McMurdo"),
        new_name("Asia/Macau"),
        new_name("Asia/Magadan"),
        new_name("Asia/Makassar"),
        new_name("Asia/Manila"),
        new_name("Asia/Muscat"),
        new_name("Atlantic/Madeira"),
        new_name("Australia/Melbourne"),
        new_name("Europe/Madrid"),
        new_name("Europe/Malta"),
        new_name("Europe/Mariehamn"),
        new_name("Europe/Minsk"),
        new_name("Europe/Monaco"),
        new_name("Europe/Moscow"),
        new_name("Indian/Mahe"),
        new_name("Indian/Maldives"),
        new_name("Indian/Mauritius"),
        new_name("Indian/Mayotte"),
        new_name("Pacific/Majuro"),
        new_name("Pacific/Marquesas"),
        new_name("Pacific/Midway"),
    ]
}

fn n() -> Vec<Name> {
    vec![
        new_name("Africa/Nairobi"),
        new_name("Africa/Ndjamena"),
        new_name("Africa/Niamey"),
        new_name("Africa/Nouakchott"),
        new_name("America/Nassau"),
        new_name("America/New_York"),
        new_name("America/Nipigon"),
        new_name("America/Nome"),
        new_name("America/Noronha"),
        new_name("America/North_Dakota/New_Salem"),
        new_name("Asia/Nicosia"),
        new_name("Asia/Novokuznetsk"),
        new_name("Asia/Novosibirsk"),
        new_name("Pacific/Nauru"),
        new_name("Pacific/Niue"),
        new_name("Pacific/Norfolk"),
        new_name("Pacific/Noumea"),
    ]
}

fn o() -> Vec<Name> {
    vec![
        new_name("Africa/Ouagadougou"),
        new_name("America/Ojinaga"),
        new_name("Asia/Omsk"),
        new_name("Asia/Oral"),
        new_name("Europe/Oslo"),
    ]
}

fn p() -> Vec<Name> {
    vec![
        new_name("Africa/Porto-Novo"),
        new_name("America/Indiana/Petersburg"),
        new_name("America/Panama"),
        new_name("America/Pangnirtung"),
        new_name("America/Paramaribo"),
        new_name("America/Phoenix"),
        new_name("America/Port-au-Prince"),
        new_name("America/Port_of_Spain"),
        new_name("America/Porto_Velho"),
        new_name("America/Puerto_Rico"),
        new_name("America/Punta_Arenas"),
        new_name("Antarctica/Palmer"),
        new_name("Asia/Phnom_Penh"),
        new_name("Asia/Pontianak"),
        new_name("Asia/Pyongyang"),
        new_name("Australia/Perth"),
        new_name("Europe/Paris"),
        new_name("Europe/Podgorica"),
        new_name("Europe/Prague"),
        new_name("Pacific/Pago_Pago"),
        new_name("Pacific/Palau"),
        new_name("Pacific/Pitcairn"),
        new_name("Pacific/Pohnpei"),
        new_name("Pacific/Port_Moresby"),
    ]
}

fn q() -> Vec<Name> {
    vec![new_name("Asia/Qatar"), new_name("Asia/Qostanay"), new_name("Asia/Qyzylorda")]
}

fn r() -> Vec<Name> {
    vec![
        new_name("America/Argentina/Rio_Gallegos"),
        new_name("America/Rainy_River"),
        new_name("America/Rankin_Inlet"),
        new_name("America/Recife"),
        new_name("America/Regina"),
        new_name("America/Resolute"),
        new_name("America/Rio_Branco"),
        new_name("Antarctica/Rothera"),
        new_name("Asia/Riyadh"),
        new_name("Atlantic/Reykjavik"),
        new_name("Europe/Riga"),
        new_name("Europe/Rome"),
        new_name("Indian/Reunion"),
        new_name("Pacific/Rarotonga"),
    ]
}

fn s() -> Vec<Name> {
    vec![
        new_name("Africa/Sao_Tome"),
        new_name("America/Argentina/Salta"),
        new_name("America/Argentina/San_Juan"),
        new_name("America/Argentina/San_Luis"),
        new_name("America/Santarem"),
        new_name("America/Santiago"),
        new_name("America/Santo_Domingo"),
        new_name("America/Sao_Paulo"),
        new_name("America/Scoresbysund"),
        new_name("America/Sitka"),
        new_name("America/St_Barthelemy"),
        new_name("America/St_Johns"),
        new_name("America/St_Kitts"),
        new_name("America/St_Lucia"),
        new_name("America/St_Thomas"),
        new_name("America/St_Vincent"),
        new_name("America/Swift_Current"),
        new_name("Antarctica/Syowa"),
        new_name("Asia/Sakhalin"),
        new_name("Asia/Samarkand"),
        new_name("Asia/Seoul"),
        new_name("Asia/Shanghai"),
        new_name("Asia/Singapore"),
        new_name("Asia/Srednekolymsk"),
        new_name("Atlantic/South_Georgia"),
        new_name("Atlantic/St_Helena"),
        new_name("Atlantic/Stanley"),
        new_name("Australia/Sydney"),
        new_name("Europe/Samara"),
        new_name("Europe/San_Marino"),
        new_name("Europe/Sarajevo"),
        new_name("Europe/Saratov"),
        new_name("Europe/Simferopol"),
        new_name("Europe/Skopje"),
        new_name("Europe/Sofia"),
        new_name("Europe/Stockholm"),
        new_name("Pacific/Saipan"),
    ]
}

fn t() -> Vec<Name> {
    vec![
        new_name("Africa/Tripoli"),
        new_name("Africa/Tunis"),
        new_name("America/Argentina/Tucuman"),
        new_name("America/Indiana/Tell_City"),
        new_name("America/Tegucigalpa"),
        new_name("America/Thule"),
        new_name("America/Thunder_Bay"),
        new_name("America/Tijuana"),
        new_name("America/Toronto"),
        new_name("America/Tortola"),
        new_name("Antarctica/Troll"),
        new_name("Asia/Taipei"),
        new_name("Asia/Tashkent"),
        new_name("Asia/Tbilisi"),
        new_name("Asia/Tehran"),
        new_name("Asia/Thimphu"),
        new_name("Asia/Tokyo"),
        new_name("Asia/Tomsk"),
        new_name("Europe/Tallinn"),
        new_name("Europe/Tirane"),
        new_name("Pacific/Tahiti"),
        new_name("Pacific/Tarawa"),
        new_name("Pacific/Tongatapu"),
    ]
}

fn u() -> Vec<Name> {
    vec![
        new_name("America/Argentina/Ushuaia"),
        new_name("Asia/Ulaanbaatar"),
        new_name("Asia/Urumqi"),
        new_name("Asia/Ust-Nera"),
        new_name("Europe/Ulyanovsk"),
        new_name("Europe/Uzhgorod"),
    ]
}

fn v() -> Vec<Name> {
    vec![
        new_name("America/Indiana/Vevay"),
        new_name("America/Indiana/Vincennes"),
        new_name("America/Vancouver"),
        new_name("Antarctica/Vostok"),
        new_name("Asia/Vientiane"),
        new_name("Asia/Vladivostok"),
        new_name("Europe/Vaduz"),
        new_name("Europe/Vatican"),
        new_name("Europe/Vienna"),
        new_name("Europe/Vilnius"),
        new_name("Europe/Volgograd"),
    ]
}

fn w() -> Vec<Name> {
    vec![
        new_name("Africa/Windhoek"),
        new_name("America/Indiana/Winamac"),
        new_name("America/Whitehorse"),
        new_name("America/Winnipeg"),
        new_name("Europe/Warsaw"),
        new_name("Pacific/Wake"),
        new_name("Pacific/Wallis"),
    ]
}

fn x() -> Vec<Name> {
    vec![]
}

fn y() -> Vec<Name> {
    vec![
        new_name("America/Yakutat"),
        new_name("America/Yellowknife"),
        new_name("Asia/Yakutsk"),
        new_name("Asia/Yangon"),
        new_name("Asia/Yekaterinburg"),
        new_name("Asia/Yerevan"),
    ]
}

fn z() -> Vec<Name> {
    vec![new_name("Europe/Zagreb"), new_name("Europe/Zaporozhye"), new_name("Europe/Zurich")]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_tokyo() {
        let r = search("tok");
        assert_eq!(1, r.len());
        assert_eq!("Asia/Tokyo", r[0]);
    }
}
