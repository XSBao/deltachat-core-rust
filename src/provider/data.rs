// file generated by src/provider/update.py

use crate::provider::Protocol::*;
use crate::provider::Socket::*;
use crate::provider::UsernamePattern::*;
use crate::provider::*;
use std::collections::HashMap;

use once_cell::sync::Lazy;

// aktivix.org.md: aktivix.org
static P_AKTIVIX_ORG: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/aktivix-org",
    server: vec![
        Server {
            protocol: IMAP,
            socket: STARTTLS,
            hostname: "newyear.aktivix.org",
            port: 143,
            username_pattern: EMAIL,
        },
        Server {
            protocol: SMTP,
            socket: STARTTLS,
            hostname: "newyear.aktivix.org",
            port: 25,
            username_pattern: EMAIL,
        },
    ],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// aol.md: aol.com
static P_AOL: Lazy<Provider> = Lazy::new(|| {
    Provider {
    status: Status::PREPARATION,
    before_login_hint: "To log in to AOL with Delta Chat, you need to set up an app password in the AOL web interface.",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/aol",
    server: vec![
    ],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
}
});

// arcor.de.md: arcor.de
static P_ARCOR_DE: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/arcor-de",
    server: vec![
        Server {
            protocol: IMAP,
            socket: SSL,
            hostname: "imap.arcor.de",
            port: 993,
            username_pattern: EMAIL,
        },
        Server {
            protocol: SMTP,
            socket: SSL,
            hostname: "mail.arcor.de",
            port: 465,
            username_pattern: EMAIL,
        },
    ],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// autistici.org.md: autistici.org
static P_AUTISTICI_ORG: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/autistici-org",
    server: vec![
        Server {
            protocol: IMAP,
            socket: SSL,
            hostname: "mail.autistici.org",
            port: 993,
            username_pattern: EMAIL,
        },
        Server {
            protocol: SMTP,
            socket: SSL,
            hostname: "smtp.autistici.org",
            port: 465,
            username_pattern: EMAIL,
        },
    ],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// bluewin.ch.md: bluewin.ch
static P_BLUEWIN_CH: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/bluewin-ch",
    server: vec![
        Server {
            protocol: IMAP,
            socket: SSL,
            hostname: "imaps.bluewin.ch",
            port: 993,
            username_pattern: EMAIL,
        },
        Server {
            protocol: SMTP,
            socket: SSL,
            hostname: "smtpauths.bluewin.ch",
            port: 465,
            username_pattern: EMAIL,
        },
    ],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// buzon.uy.md: buzon.uy
static P_BUZON_UY: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/buzon-uy",
    server: vec![
        Server {
            protocol: IMAP,
            socket: STARTTLS,
            hostname: "mail.buzon.uy",
            port: 143,
            username_pattern: EMAIL,
        },
        Server {
            protocol: SMTP,
            socket: STARTTLS,
            hostname: "mail.buzon.uy",
            port: 587,
            username_pattern: EMAIL,
        },
    ],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// chello.at.md: chello.at
static P_CHELLO_AT: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/chello-at",
    server: vec![
        Server {
            protocol: IMAP,
            socket: SSL,
            hostname: "mail.mymagenta.at",
            port: 993,
            username_pattern: EMAIL,
        },
        Server {
            protocol: SMTP,
            socket: SSL,
            hostname: "mail.mymagenta.at",
            port: 465,
            username_pattern: EMAIL,
        },
    ],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// comcast.md: xfinity.com, comcast.net
static P_COMCAST: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/comcast",
    server: vec![],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// dismail.de.md: dismail.de
static P_DISMAIL_DE: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/dismail-de",
    server: vec![],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// disroot.md: disroot.org
static P_DISROOT: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/disroot",
    server: vec![],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// dubby.org.md: dubby.org
static P_DUBBY_ORG: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/dubby-org",
    server: vec![
        Server {
            protocol: IMAP,
            socket: SSL,
            hostname: "dubby.org",
            port: 993,
            username_pattern: EMAIL,
        },
        Server {
            protocol: SMTP,
            socket: STARTTLS,
            hostname: "dubby.org",
            port: 587,
            username_pattern: EMAIL,
        },
        Server {
            protocol: SMTP,
            socket: SSL,
            hostname: "dubby.org",
            port: 465,
            username_pattern: EMAIL,
        },
    ],
    config_defaults: Some(vec![
        ConfigDefault {
            key: Config::BccSelf,
            value: "1",
        },
        ConfigDefault {
            key: Config::SentboxWatch,
            value: "0",
        },
        ConfigDefault {
            key: Config::MvboxWatch,
            value: "0",
        },
        ConfigDefault {
            key: Config::MvboxMove,
            value: "0",
        },
    ]),
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// example.com.md: example.com, example.org
static P_EXAMPLE_COM: Lazy<Provider> = Lazy::new(|| {
    Provider {
    status: Status::BROKEN,
    before_login_hint: "Hush this provider doesn't exist!",
    after_login_hint: "This provider doesn't really exist, so you can't use it :/ If you need an email provider for Delta Chat, take a look at providers.delta.chat!",
    overview_page: "https://providers.delta.chat/example-com",
    server: vec![
        Server { protocol: IMAP, socket: SSL, hostname: "imap.example.com", port: 1337, username_pattern: EMAIL },
        Server { protocol: SMTP, socket: STARTTLS, hostname: "smtp.example.com", port: 1337, username_pattern: EMAIL },
    ],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
}
});

// fastmail.md: fastmail.com
static P_FASTMAIL: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::PREPARATION,
    before_login_hint:
        "You must create an app-specific password for Delta Chat before you can log in.",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/fastmail",
    server: vec![],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// firemail.de.md: firemail.at, firemail.de
static P_FIREMAIL_DE: Lazy<Provider> = Lazy::new(|| {
    Provider {
    status: Status::PREPARATION,
    before_login_hint: "Firemail erlaubt nur bei bezahlten Accounts den vollen Zugriff auf das E-Mail-Protokoll. Wenn Sie nicht für Firemail bezahlen, verwenden Sie bitte einen anderen E-Mail-Anbieter.",
    after_login_hint: "Leider schränkt Firemail die maximale Gruppengröße ein. Je nach Bezahlmodell sind nur 5 bis 30 Gruppenmitglieder erlaubt.",
    overview_page: "https://providers.delta.chat/firemail-de",
    server: vec![
    ],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
}
});

// five.chat.md: five.chat
static P_FIVE_CHAT: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/five-chat",
    server: vec![],
    config_defaults: Some(vec![
        ConfigDefault {
            key: Config::BccSelf,
            value: "1",
        },
        ConfigDefault {
            key: Config::SentboxWatch,
            value: "0",
        },
        ConfigDefault {
            key: Config::MvboxWatch,
            value: "0",
        },
        ConfigDefault {
            key: Config::MvboxMove,
            value: "0",
        },
    ]),
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// freenet.de.md: freenet.de
static P_FREENET_DE: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/freenet-de",
    server: vec![
        Server {
            protocol: IMAP,
            socket: SSL,
            hostname: "mx.freenet.de",
            port: 993,
            username_pattern: EMAIL,
        },
        Server {
            protocol: SMTP,
            socket: STARTTLS,
            hostname: "mx.freenet.de",
            port: 587,
            username_pattern: EMAIL,
        },
    ],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// gmail.md: gmail.com, googlemail.com
static P_GMAIL: Lazy<Provider> = Lazy::new(|| {
    Provider {
    status: Status::PREPARATION,
    before_login_hint: "For Gmail accounts, you need to create an app-password if you have \"2-Step Verification\" enabled. If this setting is not available, you need to enable \"less secure apps\".",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/gmail",
    server: vec![
        Server { protocol: IMAP, socket: SSL, hostname: "imap.gmail.com", port: 993, username_pattern: EMAIL },
        Server { protocol: SMTP, socket: SSL, hostname: "smtp.gmail.com", port: 465, username_pattern: EMAIL },
    ],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: Some(Oauth2Authorizer::Gmail),
}
});

// gmx.net.md: gmx.net, gmx.de, gmx.at, gmx.ch, gmx.org, gmx.eu, gmx.info, gmx.biz, gmx.com
static P_GMX_NET: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::PREPARATION,
    before_login_hint: "You must allow IMAP access to your account before you can login.",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/gmx-net",
    server: vec![
        Server {
            protocol: IMAP,
            socket: SSL,
            hostname: "imap.gmx.net",
            port: 993,
            username_pattern: EMAIL,
        },
        Server {
            protocol: SMTP,
            socket: SSL,
            hostname: "mail.gmx.net",
            port: 465,
            username_pattern: EMAIL,
        },
        Server {
            protocol: SMTP,
            socket: STARTTLS,
            hostname: "mail.gmx.net",
            port: 587,
            username_pattern: EMAIL,
        },
    ],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// hermes.radio.md: hermes.radio
static P_HERMES_RADIO: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/hermes-radio",
    server: vec![],
    config_defaults: Some(vec![
        ConfigDefault {
            key: Config::MdnsEnabled,
            value: "0",
        },
        ConfigDefault {
            key: Config::E2eeEnabled,
            value: "0",
        },
        ConfigDefault {
            key: Config::MediaQuality,
            value: "1",
        },
        ConfigDefault {
            key: Config::ShowEmails,
            value: "2",
        },
    ]),
    strict_tls: false,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// hey.com.md: hey.com
static P_HEY_COM: Lazy<Provider> = Lazy::new(|| {
    Provider {
    status: Status::BROKEN,
    before_login_hint: "hey.com does not offer the standard IMAP e-mail protocol, so you cannot log in with Delta Chat to hey.com.",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/hey-com",
    server: vec![
    ],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
}
});

// i.ua.md: i.ua
static P_I_UA: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/i-ua",
    server: vec![],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// icloud.md: icloud.com, me.com, mac.com
static P_ICLOUD: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::PREPARATION,
    before_login_hint:
        "You must create an app-specific password for Delta Chat before you can login.",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/icloud",
    server: vec![
        Server {
            protocol: IMAP,
            socket: SSL,
            hostname: "imap.mail.me.com",
            port: 993,
            username_pattern: EMAILLOCALPART,
        },
        Server {
            protocol: SMTP,
            socket: STARTTLS,
            hostname: "smtp.mail.me.com",
            port: 587,
            username_pattern: EMAIL,
        },
    ],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// kolst.com.md: kolst.com
static P_KOLST_COM: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/kolst-com",
    server: vec![],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// kontent.com.md: kontent.com
static P_KONTENT_COM: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/kontent-com",
    server: vec![],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// mail.ru.md: mail.ru, inbox.ru, bk.ru, list.ru
static P_MAIL_RU: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/mail-ru",
    server: vec![],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// mailbox.org.md: mailbox.org, secure.mailbox.org
static P_MAILBOX_ORG: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/mailbox-org",
    server: vec![],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// nauta.cu.md: nauta.cu
static P_NAUTA_CU: Lazy<Provider> = Lazy::new(|| {
    Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "Atención - con nauta.cu, puede enviar mensajes sólo a un máximo de 20 personas a la vez. En grupos más grandes, no puede enviar mensajes o abandonar el grupo.",
    overview_page: "https://providers.delta.chat/nauta-cu",
    server: vec![
        Server { protocol: IMAP, socket: STARTTLS, hostname: "imap.nauta.cu", port: 143, username_pattern: EMAIL },
        Server { protocol: SMTP, socket: STARTTLS, hostname: "smtp.nauta.cu", port: 25, username_pattern: EMAIL },
    ],
    config_defaults: Some(vec![
        ConfigDefault { key: Config::DeleteServerAfter, value: "1" },
        ConfigDefault { key: Config::BccSelf, value: "0" },
        ConfigDefault { key: Config::SentboxWatch, value: "0" },
        ConfigDefault { key: Config::MvboxWatch, value: "0" },
        ConfigDefault { key: Config::MvboxMove, value: "0" },
        ConfigDefault { key: Config::E2eeEnabled, value: "0" },
        ConfigDefault { key: Config::MediaQuality, value: "1" },
        ConfigDefault { key: Config::FetchExistingMsgs, value: "0" },
    ]),
    strict_tls: true,
    max_smtp_rcpt_to: Some(20),
    oauth2_authorizer: None,
}
});

// outlook.com.md: hotmail.com, outlook.com, office365.com, outlook.com.tr, live.com
static P_OUTLOOK_COM: Lazy<Provider> = Lazy::new(|| {
    Provider {
    status: Status::BROKEN,
    before_login_hint: "Outlook.com email addresses will not work as expected as these servers remove some important transport information. Hopefully sooner or later there will be a fix, for now we suggest to use another email address.",
    after_login_hint: "Outlook.com email addresses will not work as expected as these servers remove some important transport information. Unencrypted 1-on-1 chats kind of work, but groups and encryption don't. Hopefully sooner or later there will be a fix, for now we suggest to use another email address.",
    overview_page: "https://providers.delta.chat/outlook-com",
    server: vec![
        Server { protocol: IMAP, socket: SSL, hostname: "imap-mail.outlook.com", port: 993, username_pattern: EMAIL },
        Server { protocol: SMTP, socket: STARTTLS, hostname: "smtp-mail.outlook.com", port: 587, username_pattern: EMAIL },
    ],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
}
});

// posteo.md: posteo.de, posteo.af, posteo.at, posteo.be, posteo.ch, posteo.cl, posteo.co, posteo.co.uk, posteo.com.br, posteo.cr, posteo.cz, posteo.dk, posteo.ee, posteo.es, posteo.eu, posteo.fi, posteo.gl, posteo.gr, posteo.hn, posteo.hr, posteo.hu, posteo.ie, posteo.in, posteo.is, posteo.jp, posteo.la, posteo.li, posteo.lt, posteo.lu, posteo.me, posteo.mx, posteo.my, posteo.net, posteo.nl, posteo.no, posteo.nz, posteo.org, posteo.pe, posteo.pl, posteo.pm, posteo.pt, posteo.ro, posteo.ru, posteo.se, posteo.sg, posteo.si, posteo.tn, posteo.uk, posteo.us
static P_POSTEO: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/posteo",
    server: vec![
        Server {
            protocol: IMAP,
            socket: STARTTLS,
            hostname: "posteo.de",
            port: 143,
            username_pattern: EMAIL,
        },
        Server {
            protocol: SMTP,
            socket: STARTTLS,
            hostname: "posteo.de",
            port: 587,
            username_pattern: EMAIL,
        },
    ],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// protonmail.md: protonmail.com, protonmail.ch
static P_PROTONMAIL: Lazy<Provider> = Lazy::new(|| {
    Provider {
    status: Status::BROKEN,
    before_login_hint: "Protonmail does not offer the standard IMAP e-mail protocol, so you cannot log in with Delta Chat to Protonmail.",
    after_login_hint: "To use Delta Chat with Protonmail, the IMAP bridge must be running in the background. If you have connectivity issues, double check whether it works as expected.",
    overview_page: "https://providers.delta.chat/protonmail",
    server: vec![
    ],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
}
});

// riseup.net.md: riseup.net
static P_RISEUP_NET: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/riseup-net",
    server: vec![],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// rogers.com.md: rogers.com
static P_ROGERS_COM: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/rogers-com",
    server: vec![],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// systemli.org.md: systemli.org
static P_SYSTEMLI_ORG: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/systemli-org",
    server: vec![],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// t-online.md: t-online.de, magenta.de
static P_T_ONLINE: Lazy<Provider> = Lazy::new(|| {
    Provider {
    status: Status::PREPARATION,
    before_login_hint: "To use Delta Chat with a T-Online email address, you need to create an app password in the web interface.",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/t-online",
    server: vec![
    ],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
}
});

// testrun.md: testrun.org
static P_TESTRUN: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/testrun",
    server: vec![
        Server {
            protocol: IMAP,
            socket: SSL,
            hostname: "testrun.org",
            port: 993,
            username_pattern: EMAIL,
        },
        Server {
            protocol: IMAP,
            socket: STARTTLS,
            hostname: "testrun.org",
            port: 143,
            username_pattern: EMAIL,
        },
        Server {
            protocol: SMTP,
            socket: STARTTLS,
            hostname: "testrun.org",
            port: 587,
            username_pattern: EMAIL,
        },
    ],
    config_defaults: Some(vec![
        ConfigDefault {
            key: Config::BccSelf,
            value: "1",
        },
        ConfigDefault {
            key: Config::SentboxWatch,
            value: "0",
        },
        ConfigDefault {
            key: Config::MvboxWatch,
            value: "0",
        },
        ConfigDefault {
            key: Config::MvboxMove,
            value: "0",
        },
    ]),
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// tiscali.it.md: tiscali.it
static P_TISCALI_IT: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/tiscali-it",
    server: vec![
        Server {
            protocol: IMAP,
            socket: SSL,
            hostname: "imap.tiscali.it",
            port: 993,
            username_pattern: EMAIL,
        },
        Server {
            protocol: SMTP,
            socket: SSL,
            hostname: "smtp.tiscali.it",
            port: 465,
            username_pattern: EMAIL,
        },
    ],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// ukr.net.md: ukr.net
static P_UKR_NET: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/ukr-net",
    server: vec![],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// undernet.uy.md: undernet.uy
static P_UNDERNET_UY: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/undernet-uy",
    server: vec![
        Server {
            protocol: IMAP,
            socket: STARTTLS,
            hostname: "undernet.uy",
            port: 143,
            username_pattern: EMAIL,
        },
        Server {
            protocol: SMTP,
            socket: STARTTLS,
            hostname: "undernet.uy",
            port: 587,
            username_pattern: EMAIL,
        },
    ],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// vfemail.md: vfemail.net
static P_VFEMAIL: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/vfemail",
    server: vec![],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// vodafone.de.md: vodafone.de, vodafonemail.de
static P_VODAFONE_DE: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/vodafone-de",
    server: vec![
        Server {
            protocol: IMAP,
            socket: SSL,
            hostname: "imap.vodafonemail.de",
            port: 993,
            username_pattern: EMAIL,
        },
        Server {
            protocol: SMTP,
            socket: STARTTLS,
            hostname: "smtp.vodafonemail.de",
            port: 587,
            username_pattern: EMAIL,
        },
    ],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

// web.de.md: web.de, email.de, flirt.ms, hallo.ms, kuss.ms, love.ms, magic.ms, singles.ms, cool.ms, kanzler.ms, okay.ms, party.ms, pop.ms, stars.ms, techno.ms, clever.ms, deutschland.ms, genial.ms, ich.ms, online.ms, smart.ms, wichtig.ms, action.ms, fussball.ms, joker.ms, planet.ms, power.ms
static P_WEB_DE: Lazy<Provider> = Lazy::new(|| {
    Provider {
    status: Status::PREPARATION,
    before_login_hint: "You must allow IMAP access to your account before you can login.",
    after_login_hint: "Note: if you have your web.de spam settings too strict, you won't receive contact requests from new people. If you want to receive contact requests, you should disable the \"3-Wege-Spamschutz\" in the web.de settings.  Read how: https://hilfe.web.de/email/spam-und-viren/spamschutz-einstellungen.html",
    overview_page: "https://providers.delta.chat/web-de",
    server: vec![
        Server { protocol: IMAP, socket: SSL, hostname: "imap.web.de", port: 993, username_pattern: EMAILLOCALPART },
        Server { protocol: IMAP, socket: STARTTLS, hostname: "imap.web.de", port: 143, username_pattern: EMAILLOCALPART },
        Server { protocol: SMTP, socket: STARTTLS, hostname: "smtp.web.de", port: 587, username_pattern: EMAILLOCALPART },
    ],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
}
});

// yahoo.md: yahoo.com, yahoo.de, yahoo.it, yahoo.fr, yahoo.es, yahoo.se, yahoo.co.uk, yahoo.co.nz, yahoo.com.au, yahoo.com.ar, yahoo.com.br, yahoo.com.mx, ymail.com, rocketmail.com, yahoodns.net
static P_YAHOO: Lazy<Provider> = Lazy::new(|| {
    Provider {
    status: Status::PREPARATION,
    before_login_hint: "To use Delta Chat with your Yahoo email address you have to create an \"App-Password\" in the account security screen.",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/yahoo",
    server: vec![
        Server { protocol: IMAP, socket: SSL, hostname: "imap.mail.yahoo.com", port: 993, username_pattern: EMAIL },
        Server { protocol: SMTP, socket: SSL, hostname: "smtp.mail.yahoo.com", port: 465, username_pattern: EMAIL },
    ],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
}
});

// yandex.ru.md: yandex.com, yandex.by, yandex.kz, yandex.ru, yandex.ua, ya.ru, narod.ru
static P_YANDEX_RU: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::PREPARATION,
    before_login_hint: "For Yandex accounts, you have to set IMAP protocol option turned on.",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/yandex-ru",
    server: vec![
        Server {
            protocol: IMAP,
            socket: SSL,
            hostname: "imap.yandex.com",
            port: 993,
            username_pattern: EMAIL,
        },
        Server {
            protocol: SMTP,
            socket: SSL,
            hostname: "smtp.yandex.com",
            port: 465,
            username_pattern: EMAIL,
        },
    ],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: Some(Oauth2Authorizer::Yandex),
});

// ziggo.nl.md: ziggo.nl
static P_ZIGGO_NL: Lazy<Provider> = Lazy::new(|| Provider {
    status: Status::OK,
    before_login_hint: "",
    after_login_hint: "",
    overview_page: "https://providers.delta.chat/ziggo-nl",
    server: vec![
        Server {
            protocol: IMAP,
            socket: SSL,
            hostname: "imap.ziggo.nl",
            port: 993,
            username_pattern: EMAIL,
        },
        Server {
            protocol: SMTP,
            socket: STARTTLS,
            hostname: "smtp.ziggo.nl",
            port: 587,
            username_pattern: EMAIL,
        },
    ],
    config_defaults: None,
    strict_tls: true,
    max_smtp_rcpt_to: None,
    oauth2_authorizer: None,
});

pub static PROVIDER_DATA: Lazy<HashMap<&'static str, &'static Provider>> = Lazy::new(|| {
    [
        ("aktivix.org", &*P_AKTIVIX_ORG),
        ("aol.com", &*P_AOL),
        ("arcor.de", &*P_ARCOR_DE),
        ("autistici.org", &*P_AUTISTICI_ORG),
        ("bluewin.ch", &*P_BLUEWIN_CH),
        ("buzon.uy", &*P_BUZON_UY),
        ("chello.at", &*P_CHELLO_AT),
        ("xfinity.com", &*P_COMCAST),
        ("comcast.net", &*P_COMCAST),
        ("dismail.de", &*P_DISMAIL_DE),
        ("disroot.org", &*P_DISROOT),
        ("dubby.org", &*P_DUBBY_ORG),
        ("example.com", &*P_EXAMPLE_COM),
        ("example.org", &*P_EXAMPLE_COM),
        ("fastmail.com", &*P_FASTMAIL),
        ("firemail.at", &*P_FIREMAIL_DE),
        ("firemail.de", &*P_FIREMAIL_DE),
        ("five.chat", &*P_FIVE_CHAT),
        ("freenet.de", &*P_FREENET_DE),
        ("gmail.com", &*P_GMAIL),
        ("googlemail.com", &*P_GMAIL),
        ("gmx.net", &*P_GMX_NET),
        ("gmx.de", &*P_GMX_NET),
        ("gmx.at", &*P_GMX_NET),
        ("gmx.ch", &*P_GMX_NET),
        ("gmx.org", &*P_GMX_NET),
        ("gmx.eu", &*P_GMX_NET),
        ("gmx.info", &*P_GMX_NET),
        ("gmx.biz", &*P_GMX_NET),
        ("gmx.com", &*P_GMX_NET),
        ("hermes.radio", &*P_HERMES_RADIO),
        ("hey.com", &*P_HEY_COM),
        ("i.ua", &*P_I_UA),
        ("icloud.com", &*P_ICLOUD),
        ("me.com", &*P_ICLOUD),
        ("mac.com", &*P_ICLOUD),
        ("kolst.com", &*P_KOLST_COM),
        ("kontent.com", &*P_KONTENT_COM),
        ("mail.ru", &*P_MAIL_RU),
        ("inbox.ru", &*P_MAIL_RU),
        ("bk.ru", &*P_MAIL_RU),
        ("list.ru", &*P_MAIL_RU),
        ("mailbox.org", &*P_MAILBOX_ORG),
        ("secure.mailbox.org", &*P_MAILBOX_ORG),
        ("nauta.cu", &*P_NAUTA_CU),
        ("hotmail.com", &*P_OUTLOOK_COM),
        ("outlook.com", &*P_OUTLOOK_COM),
        ("office365.com", &*P_OUTLOOK_COM),
        ("outlook.com.tr", &*P_OUTLOOK_COM),
        ("live.com", &*P_OUTLOOK_COM),
        ("posteo.de", &*P_POSTEO),
        ("posteo.af", &*P_POSTEO),
        ("posteo.at", &*P_POSTEO),
        ("posteo.be", &*P_POSTEO),
        ("posteo.ch", &*P_POSTEO),
        ("posteo.cl", &*P_POSTEO),
        ("posteo.co", &*P_POSTEO),
        ("posteo.co.uk", &*P_POSTEO),
        ("posteo.com.br", &*P_POSTEO),
        ("posteo.cr", &*P_POSTEO),
        ("posteo.cz", &*P_POSTEO),
        ("posteo.dk", &*P_POSTEO),
        ("posteo.ee", &*P_POSTEO),
        ("posteo.es", &*P_POSTEO),
        ("posteo.eu", &*P_POSTEO),
        ("posteo.fi", &*P_POSTEO),
        ("posteo.gl", &*P_POSTEO),
        ("posteo.gr", &*P_POSTEO),
        ("posteo.hn", &*P_POSTEO),
        ("posteo.hr", &*P_POSTEO),
        ("posteo.hu", &*P_POSTEO),
        ("posteo.ie", &*P_POSTEO),
        ("posteo.in", &*P_POSTEO),
        ("posteo.is", &*P_POSTEO),
        ("posteo.jp", &*P_POSTEO),
        ("posteo.la", &*P_POSTEO),
        ("posteo.li", &*P_POSTEO),
        ("posteo.lt", &*P_POSTEO),
        ("posteo.lu", &*P_POSTEO),
        ("posteo.me", &*P_POSTEO),
        ("posteo.mx", &*P_POSTEO),
        ("posteo.my", &*P_POSTEO),
        ("posteo.net", &*P_POSTEO),
        ("posteo.nl", &*P_POSTEO),
        ("posteo.no", &*P_POSTEO),
        ("posteo.nz", &*P_POSTEO),
        ("posteo.org", &*P_POSTEO),
        ("posteo.pe", &*P_POSTEO),
        ("posteo.pl", &*P_POSTEO),
        ("posteo.pm", &*P_POSTEO),
        ("posteo.pt", &*P_POSTEO),
        ("posteo.ro", &*P_POSTEO),
        ("posteo.ru", &*P_POSTEO),
        ("posteo.se", &*P_POSTEO),
        ("posteo.sg", &*P_POSTEO),
        ("posteo.si", &*P_POSTEO),
        ("posteo.tn", &*P_POSTEO),
        ("posteo.uk", &*P_POSTEO),
        ("posteo.us", &*P_POSTEO),
        ("protonmail.com", &*P_PROTONMAIL),
        ("protonmail.ch", &*P_PROTONMAIL),
        ("riseup.net", &*P_RISEUP_NET),
        ("rogers.com", &*P_ROGERS_COM),
        ("systemli.org", &*P_SYSTEMLI_ORG),
        ("t-online.de", &*P_T_ONLINE),
        ("magenta.de", &*P_T_ONLINE),
        ("testrun.org", &*P_TESTRUN),
        ("tiscali.it", &*P_TISCALI_IT),
        ("ukr.net", &*P_UKR_NET),
        ("undernet.uy", &*P_UNDERNET_UY),
        ("vfemail.net", &*P_VFEMAIL),
        ("vodafone.de", &*P_VODAFONE_DE),
        ("vodafonemail.de", &*P_VODAFONE_DE),
        ("web.de", &*P_WEB_DE),
        ("email.de", &*P_WEB_DE),
        ("flirt.ms", &*P_WEB_DE),
        ("hallo.ms", &*P_WEB_DE),
        ("kuss.ms", &*P_WEB_DE),
        ("love.ms", &*P_WEB_DE),
        ("magic.ms", &*P_WEB_DE),
        ("singles.ms", &*P_WEB_DE),
        ("cool.ms", &*P_WEB_DE),
        ("kanzler.ms", &*P_WEB_DE),
        ("okay.ms", &*P_WEB_DE),
        ("party.ms", &*P_WEB_DE),
        ("pop.ms", &*P_WEB_DE),
        ("stars.ms", &*P_WEB_DE),
        ("techno.ms", &*P_WEB_DE),
        ("clever.ms", &*P_WEB_DE),
        ("deutschland.ms", &*P_WEB_DE),
        ("genial.ms", &*P_WEB_DE),
        ("ich.ms", &*P_WEB_DE),
        ("online.ms", &*P_WEB_DE),
        ("smart.ms", &*P_WEB_DE),
        ("wichtig.ms", &*P_WEB_DE),
        ("action.ms", &*P_WEB_DE),
        ("fussball.ms", &*P_WEB_DE),
        ("joker.ms", &*P_WEB_DE),
        ("planet.ms", &*P_WEB_DE),
        ("power.ms", &*P_WEB_DE),
        ("yahoo.com", &*P_YAHOO),
        ("yahoo.de", &*P_YAHOO),
        ("yahoo.it", &*P_YAHOO),
        ("yahoo.fr", &*P_YAHOO),
        ("yahoo.es", &*P_YAHOO),
        ("yahoo.se", &*P_YAHOO),
        ("yahoo.co.uk", &*P_YAHOO),
        ("yahoo.co.nz", &*P_YAHOO),
        ("yahoo.com.au", &*P_YAHOO),
        ("yahoo.com.ar", &*P_YAHOO),
        ("yahoo.com.br", &*P_YAHOO),
        ("yahoo.com.mx", &*P_YAHOO),
        ("ymail.com", &*P_YAHOO),
        ("rocketmail.com", &*P_YAHOO),
        ("yahoodns.net", &*P_YAHOO),
        ("yandex.com", &*P_YANDEX_RU),
        ("yandex.by", &*P_YANDEX_RU),
        ("yandex.kz", &*P_YANDEX_RU),
        ("yandex.ru", &*P_YANDEX_RU),
        ("yandex.ua", &*P_YANDEX_RU),
        ("ya.ru", &*P_YANDEX_RU),
        ("narod.ru", &*P_YANDEX_RU),
        ("ziggo.nl", &*P_ZIGGO_NL),
    ]
    .iter()
    .copied()
    .collect()
});

pub static PROVIDER_UPDATED: Lazy<chrono::NaiveDate> =
    Lazy::new(|| chrono::NaiveDate::from_ymd(2020, 12, 24));
