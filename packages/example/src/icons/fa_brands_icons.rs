
use crate::{IconSet, IconName};
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::*;
use dioxus_free_icons::Icon;


pub fn get_icon_set() -> IconSet{
    IconSet {
        code: "fa_brands_icons".to_string(),
        name: "Font Awesome".to_string(),
        url: "https://fontawesome.com/".to_string(),
        license: "CC BY 4.0 License".to_string(),
        license_url: "https://creativecommons.org/licenses/by/4.0/".to_string(),
        version: "6.1.1".to_string(),
        source_url: "https://github.com/FortAwesome/Font-Awesome/tree/6.1.1".to_string(),
        icons: vec![

            IconName {
                name: "Fa42Group".to_string(),
                icon: rsx!(Icon { icon: Fa42Group }),
            },

            IconName {
                name: "Fa500px".to_string(),
                icon: rsx!(Icon { icon: Fa500px }),
            },

            IconName {
                name: "FaAccessibleIcon".to_string(),
                icon: rsx!(Icon { icon: FaAccessibleIcon }),
            },

            IconName {
                name: "FaAccusoft".to_string(),
                icon: rsx!(Icon { icon: FaAccusoft }),
            },

            IconName {
                name: "FaAdn".to_string(),
                icon: rsx!(Icon { icon: FaAdn }),
            },

            IconName {
                name: "FaAdversal".to_string(),
                icon: rsx!(Icon { icon: FaAdversal }),
            },

            IconName {
                name: "FaAffiliatetheme".to_string(),
                icon: rsx!(Icon { icon: FaAffiliatetheme }),
            },

            IconName {
                name: "FaAirbnb".to_string(),
                icon: rsx!(Icon { icon: FaAirbnb }),
            },

            IconName {
                name: "FaAlgolia".to_string(),
                icon: rsx!(Icon { icon: FaAlgolia }),
            },

            IconName {
                name: "FaAlipay".to_string(),
                icon: rsx!(Icon { icon: FaAlipay }),
            },

            IconName {
                name: "FaAmazonPay".to_string(),
                icon: rsx!(Icon { icon: FaAmazonPay }),
            },

            IconName {
                name: "FaAmazon".to_string(),
                icon: rsx!(Icon { icon: FaAmazon }),
            },

            IconName {
                name: "FaAmilia".to_string(),
                icon: rsx!(Icon { icon: FaAmilia }),
            },

            IconName {
                name: "FaAndroid".to_string(),
                icon: rsx!(Icon { icon: FaAndroid }),
            },

            IconName {
                name: "FaAngellist".to_string(),
                icon: rsx!(Icon { icon: FaAngellist }),
            },

            IconName {
                name: "FaAngrycreative".to_string(),
                icon: rsx!(Icon { icon: FaAngrycreative }),
            },

            IconName {
                name: "FaAngular".to_string(),
                icon: rsx!(Icon { icon: FaAngular }),
            },

            IconName {
                name: "FaAppStoreIos".to_string(),
                icon: rsx!(Icon { icon: FaAppStoreIos }),
            },

            IconName {
                name: "FaAppStore".to_string(),
                icon: rsx!(Icon { icon: FaAppStore }),
            },

            IconName {
                name: "FaApper".to_string(),
                icon: rsx!(Icon { icon: FaApper }),
            },

            IconName {
                name: "FaApplePay".to_string(),
                icon: rsx!(Icon { icon: FaApplePay }),
            },

            IconName {
                name: "FaApple".to_string(),
                icon: rsx!(Icon { icon: FaApple }),
            },

            IconName {
                name: "FaArtstation".to_string(),
                icon: rsx!(Icon { icon: FaArtstation }),
            },

            IconName {
                name: "FaAsymmetrik".to_string(),
                icon: rsx!(Icon { icon: FaAsymmetrik }),
            },

            IconName {
                name: "FaAtlassian".to_string(),
                icon: rsx!(Icon { icon: FaAtlassian }),
            },

            IconName {
                name: "FaAudible".to_string(),
                icon: rsx!(Icon { icon: FaAudible }),
            },

            IconName {
                name: "FaAutoprefixer".to_string(),
                icon: rsx!(Icon { icon: FaAutoprefixer }),
            },

            IconName {
                name: "FaAvianex".to_string(),
                icon: rsx!(Icon { icon: FaAvianex }),
            },

            IconName {
                name: "FaAviato".to_string(),
                icon: rsx!(Icon { icon: FaAviato }),
            },

            IconName {
                name: "FaAws".to_string(),
                icon: rsx!(Icon { icon: FaAws }),
            },

            IconName {
                name: "FaBandcamp".to_string(),
                icon: rsx!(Icon { icon: FaBandcamp }),
            },

            IconName {
                name: "FaBattleNet".to_string(),
                icon: rsx!(Icon { icon: FaBattleNet }),
            },

            IconName {
                name: "FaBehanceSquare".to_string(),
                icon: rsx!(Icon { icon: FaBehanceSquare }),
            },

            IconName {
                name: "FaBehance".to_string(),
                icon: rsx!(Icon { icon: FaBehance }),
            },

            IconName {
                name: "FaBilibili".to_string(),
                icon: rsx!(Icon { icon: FaBilibili }),
            },

            IconName {
                name: "FaBimobject".to_string(),
                icon: rsx!(Icon { icon: FaBimobject }),
            },

            IconName {
                name: "FaBitbucket".to_string(),
                icon: rsx!(Icon { icon: FaBitbucket }),
            },

            IconName {
                name: "FaBitcoin".to_string(),
                icon: rsx!(Icon { icon: FaBitcoin }),
            },

            IconName {
                name: "FaBity".to_string(),
                icon: rsx!(Icon { icon: FaBity }),
            },

            IconName {
                name: "FaBlackTie".to_string(),
                icon: rsx!(Icon { icon: FaBlackTie }),
            },

            IconName {
                name: "FaBlackberry".to_string(),
                icon: rsx!(Icon { icon: FaBlackberry }),
            },

            IconName {
                name: "FaBloggerB".to_string(),
                icon: rsx!(Icon { icon: FaBloggerB }),
            },

            IconName {
                name: "FaBlogger".to_string(),
                icon: rsx!(Icon { icon: FaBlogger }),
            },

            IconName {
                name: "FaBluetoothB".to_string(),
                icon: rsx!(Icon { icon: FaBluetoothB }),
            },

            IconName {
                name: "FaBluetooth".to_string(),
                icon: rsx!(Icon { icon: FaBluetooth }),
            },

            IconName {
                name: "FaBootstrap".to_string(),
                icon: rsx!(Icon { icon: FaBootstrap }),
            },

            IconName {
                name: "FaBots".to_string(),
                icon: rsx!(Icon { icon: FaBots }),
            },

            IconName {
                name: "FaBtc".to_string(),
                icon: rsx!(Icon { icon: FaBtc }),
            },

            IconName {
                name: "FaBuffer".to_string(),
                icon: rsx!(Icon { icon: FaBuffer }),
            },

            IconName {
                name: "FaBuromobelexperte".to_string(),
                icon: rsx!(Icon { icon: FaBuromobelexperte }),
            },

            IconName {
                name: "FaBuyNLarge".to_string(),
                icon: rsx!(Icon { icon: FaBuyNLarge }),
            },

            IconName {
                name: "FaBuysellads".to_string(),
                icon: rsx!(Icon { icon: FaBuysellads }),
            },

            IconName {
                name: "FaCanadianMapleLeaf".to_string(),
                icon: rsx!(Icon { icon: FaCanadianMapleLeaf }),
            },

            IconName {
                name: "FaCcAmazonPay".to_string(),
                icon: rsx!(Icon { icon: FaCcAmazonPay }),
            },

            IconName {
                name: "FaCcAmex".to_string(),
                icon: rsx!(Icon { icon: FaCcAmex }),
            },

            IconName {
                name: "FaCcApplePay".to_string(),
                icon: rsx!(Icon { icon: FaCcApplePay }),
            },

            IconName {
                name: "FaCcDinersClub".to_string(),
                icon: rsx!(Icon { icon: FaCcDinersClub }),
            },

            IconName {
                name: "FaCcDiscover".to_string(),
                icon: rsx!(Icon { icon: FaCcDiscover }),
            },

            IconName {
                name: "FaCcJcb".to_string(),
                icon: rsx!(Icon { icon: FaCcJcb }),
            },

            IconName {
                name: "FaCcMastercard".to_string(),
                icon: rsx!(Icon { icon: FaCcMastercard }),
            },

            IconName {
                name: "FaCcPaypal".to_string(),
                icon: rsx!(Icon { icon: FaCcPaypal }),
            },

            IconName {
                name: "FaCcStripe".to_string(),
                icon: rsx!(Icon { icon: FaCcStripe }),
            },

            IconName {
                name: "FaCcVisa".to_string(),
                icon: rsx!(Icon { icon: FaCcVisa }),
            },

            IconName {
                name: "FaCentercode".to_string(),
                icon: rsx!(Icon { icon: FaCentercode }),
            },

            IconName {
                name: "FaCentos".to_string(),
                icon: rsx!(Icon { icon: FaCentos }),
            },

            IconName {
                name: "FaChrome".to_string(),
                icon: rsx!(Icon { icon: FaChrome }),
            },

            IconName {
                name: "FaChromecast".to_string(),
                icon: rsx!(Icon { icon: FaChromecast }),
            },

            IconName {
                name: "FaCloudflare".to_string(),
                icon: rsx!(Icon { icon: FaCloudflare }),
            },

            IconName {
                name: "FaCloudscale".to_string(),
                icon: rsx!(Icon { icon: FaCloudscale }),
            },

            IconName {
                name: "FaCloudsmith".to_string(),
                icon: rsx!(Icon { icon: FaCloudsmith }),
            },

            IconName {
                name: "FaCloudversify".to_string(),
                icon: rsx!(Icon { icon: FaCloudversify }),
            },

            IconName {
                name: "FaCmplid".to_string(),
                icon: rsx!(Icon { icon: FaCmplid }),
            },

            IconName {
                name: "FaCodepen".to_string(),
                icon: rsx!(Icon { icon: FaCodepen }),
            },

            IconName {
                name: "FaCodiepie".to_string(),
                icon: rsx!(Icon { icon: FaCodiepie }),
            },

            IconName {
                name: "FaConfluence".to_string(),
                icon: rsx!(Icon { icon: FaConfluence }),
            },

            IconName {
                name: "FaConnectdevelop".to_string(),
                icon: rsx!(Icon { icon: FaConnectdevelop }),
            },

            IconName {
                name: "FaContao".to_string(),
                icon: rsx!(Icon { icon: FaContao }),
            },

            IconName {
                name: "FaCottonBureau".to_string(),
                icon: rsx!(Icon { icon: FaCottonBureau }),
            },

            IconName {
                name: "FaCpanel".to_string(),
                icon: rsx!(Icon { icon: FaCpanel }),
            },

            IconName {
                name: "FaCreativeCommonsBy".to_string(),
                icon: rsx!(Icon { icon: FaCreativeCommonsBy }),
            },

            IconName {
                name: "FaCreativeCommonsNcEu".to_string(),
                icon: rsx!(Icon { icon: FaCreativeCommonsNcEu }),
            },

            IconName {
                name: "FaCreativeCommonsNcJp".to_string(),
                icon: rsx!(Icon { icon: FaCreativeCommonsNcJp }),
            },

            IconName {
                name: "FaCreativeCommonsNc".to_string(),
                icon: rsx!(Icon { icon: FaCreativeCommonsNc }),
            },

            IconName {
                name: "FaCreativeCommonsNd".to_string(),
                icon: rsx!(Icon { icon: FaCreativeCommonsNd }),
            },

            IconName {
                name: "FaCreativeCommonsPdAlt".to_string(),
                icon: rsx!(Icon { icon: FaCreativeCommonsPdAlt }),
            },

            IconName {
                name: "FaCreativeCommonsPd".to_string(),
                icon: rsx!(Icon { icon: FaCreativeCommonsPd }),
            },

            IconName {
                name: "FaCreativeCommonsRemix".to_string(),
                icon: rsx!(Icon { icon: FaCreativeCommonsRemix }),
            },

            IconName {
                name: "FaCreativeCommonsSa".to_string(),
                icon: rsx!(Icon { icon: FaCreativeCommonsSa }),
            },

            IconName {
                name: "FaCreativeCommonsSamplingPlus".to_string(),
                icon: rsx!(Icon { icon: FaCreativeCommonsSamplingPlus }),
            },

            IconName {
                name: "FaCreativeCommonsSampling".to_string(),
                icon: rsx!(Icon { icon: FaCreativeCommonsSampling }),
            },

            IconName {
                name: "FaCreativeCommonsShare".to_string(),
                icon: rsx!(Icon { icon: FaCreativeCommonsShare }),
            },

            IconName {
                name: "FaCreativeCommonsZero".to_string(),
                icon: rsx!(Icon { icon: FaCreativeCommonsZero }),
            },

            IconName {
                name: "FaCreativeCommons".to_string(),
                icon: rsx!(Icon { icon: FaCreativeCommons }),
            },

            IconName {
                name: "FaCriticalRole".to_string(),
                icon: rsx!(Icon { icon: FaCriticalRole }),
            },

            IconName {
                name: "FaCss3Alt".to_string(),
                icon: rsx!(Icon { icon: FaCss3Alt }),
            },

            IconName {
                name: "FaCss3".to_string(),
                icon: rsx!(Icon { icon: FaCss3 }),
            },

            IconName {
                name: "FaCuttlefish".to_string(),
                icon: rsx!(Icon { icon: FaCuttlefish }),
            },

            IconName {
                name: "FaDAndDBeyond".to_string(),
                icon: rsx!(Icon { icon: FaDAndDBeyond }),
            },

            IconName {
                name: "FaDAndD".to_string(),
                icon: rsx!(Icon { icon: FaDAndD }),
            },

            IconName {
                name: "FaDailymotion".to_string(),
                icon: rsx!(Icon { icon: FaDailymotion }),
            },

            IconName {
                name: "FaDashcube".to_string(),
                icon: rsx!(Icon { icon: FaDashcube }),
            },

            IconName {
                name: "FaDeezer".to_string(),
                icon: rsx!(Icon { icon: FaDeezer }),
            },

            IconName {
                name: "FaDelicious".to_string(),
                icon: rsx!(Icon { icon: FaDelicious }),
            },

            IconName {
                name: "FaDeploydog".to_string(),
                icon: rsx!(Icon { icon: FaDeploydog }),
            },

            IconName {
                name: "FaDeskpro".to_string(),
                icon: rsx!(Icon { icon: FaDeskpro }),
            },

            IconName {
                name: "FaDev".to_string(),
                icon: rsx!(Icon { icon: FaDev }),
            },

            IconName {
                name: "FaDeviantart".to_string(),
                icon: rsx!(Icon { icon: FaDeviantart }),
            },

            IconName {
                name: "FaDhl".to_string(),
                icon: rsx!(Icon { icon: FaDhl }),
            },

            IconName {
                name: "FaDiaspora".to_string(),
                icon: rsx!(Icon { icon: FaDiaspora }),
            },

            IconName {
                name: "FaDigg".to_string(),
                icon: rsx!(Icon { icon: FaDigg }),
            },

            IconName {
                name: "FaDigitalOcean".to_string(),
                icon: rsx!(Icon { icon: FaDigitalOcean }),
            },

            IconName {
                name: "FaDiscord".to_string(),
                icon: rsx!(Icon { icon: FaDiscord }),
            },

            IconName {
                name: "FaDiscourse".to_string(),
                icon: rsx!(Icon { icon: FaDiscourse }),
            },

            IconName {
                name: "FaDochub".to_string(),
                icon: rsx!(Icon { icon: FaDochub }),
            },

            IconName {
                name: "FaDocker".to_string(),
                icon: rsx!(Icon { icon: FaDocker }),
            },

            IconName {
                name: "FaDraft2digital".to_string(),
                icon: rsx!(Icon { icon: FaDraft2digital }),
            },

            IconName {
                name: "FaDribbbleSquare".to_string(),
                icon: rsx!(Icon { icon: FaDribbbleSquare }),
            },

            IconName {
                name: "FaDribbble".to_string(),
                icon: rsx!(Icon { icon: FaDribbble }),
            },

            IconName {
                name: "FaDropbox".to_string(),
                icon: rsx!(Icon { icon: FaDropbox }),
            },

            IconName {
                name: "FaDrupal".to_string(),
                icon: rsx!(Icon { icon: FaDrupal }),
            },

            IconName {
                name: "FaDyalog".to_string(),
                icon: rsx!(Icon { icon: FaDyalog }),
            },

            IconName {
                name: "FaEarlybirds".to_string(),
                icon: rsx!(Icon { icon: FaEarlybirds }),
            },

            IconName {
                name: "FaEbay".to_string(),
                icon: rsx!(Icon { icon: FaEbay }),
            },

            IconName {
                name: "FaEdgeLegacy".to_string(),
                icon: rsx!(Icon { icon: FaEdgeLegacy }),
            },

            IconName {
                name: "FaEdge".to_string(),
                icon: rsx!(Icon { icon: FaEdge }),
            },

            IconName {
                name: "FaElementor".to_string(),
                icon: rsx!(Icon { icon: FaElementor }),
            },

            IconName {
                name: "FaEllo".to_string(),
                icon: rsx!(Icon { icon: FaEllo }),
            },

            IconName {
                name: "FaEmber".to_string(),
                icon: rsx!(Icon { icon: FaEmber }),
            },

            IconName {
                name: "FaEmpire".to_string(),
                icon: rsx!(Icon { icon: FaEmpire }),
            },

            IconName {
                name: "FaEnvira".to_string(),
                icon: rsx!(Icon { icon: FaEnvira }),
            },

            IconName {
                name: "FaErlang".to_string(),
                icon: rsx!(Icon { icon: FaErlang }),
            },

            IconName {
                name: "FaEthereum".to_string(),
                icon: rsx!(Icon { icon: FaEthereum }),
            },

            IconName {
                name: "FaEtsy".to_string(),
                icon: rsx!(Icon { icon: FaEtsy }),
            },

            IconName {
                name: "FaEvernote".to_string(),
                icon: rsx!(Icon { icon: FaEvernote }),
            },

            IconName {
                name: "FaExpeditedssl".to_string(),
                icon: rsx!(Icon { icon: FaExpeditedssl }),
            },

            IconName {
                name: "FaFacebookF".to_string(),
                icon: rsx!(Icon { icon: FaFacebookF }),
            },

            IconName {
                name: "FaFacebookMessenger".to_string(),
                icon: rsx!(Icon { icon: FaFacebookMessenger }),
            },

            IconName {
                name: "FaFacebookSquare".to_string(),
                icon: rsx!(Icon { icon: FaFacebookSquare }),
            },

            IconName {
                name: "FaFacebook".to_string(),
                icon: rsx!(Icon { icon: FaFacebook }),
            },

            IconName {
                name: "FaFantasyFlightGames".to_string(),
                icon: rsx!(Icon { icon: FaFantasyFlightGames }),
            },

            IconName {
                name: "FaFedex".to_string(),
                icon: rsx!(Icon { icon: FaFedex }),
            },

            IconName {
                name: "FaFedora".to_string(),
                icon: rsx!(Icon { icon: FaFedora }),
            },

            IconName {
                name: "FaFigma".to_string(),
                icon: rsx!(Icon { icon: FaFigma }),
            },

            IconName {
                name: "FaFirefoxBrowser".to_string(),
                icon: rsx!(Icon { icon: FaFirefoxBrowser }),
            },

            IconName {
                name: "FaFirefox".to_string(),
                icon: rsx!(Icon { icon: FaFirefox }),
            },

            IconName {
                name: "FaFirstOrderAlt".to_string(),
                icon: rsx!(Icon { icon: FaFirstOrderAlt }),
            },

            IconName {
                name: "FaFirstOrder".to_string(),
                icon: rsx!(Icon { icon: FaFirstOrder }),
            },

            IconName {
                name: "FaFirstdraft".to_string(),
                icon: rsx!(Icon { icon: FaFirstdraft }),
            },

            IconName {
                name: "FaFlickr".to_string(),
                icon: rsx!(Icon { icon: FaFlickr }),
            },

            IconName {
                name: "FaFlipboard".to_string(),
                icon: rsx!(Icon { icon: FaFlipboard }),
            },

            IconName {
                name: "FaFly".to_string(),
                icon: rsx!(Icon { icon: FaFly }),
            },

            IconName {
                name: "FaFontAwesome".to_string(),
                icon: rsx!(Icon { icon: FaFontAwesome }),
            },

            IconName {
                name: "FaFonticonsFi".to_string(),
                icon: rsx!(Icon { icon: FaFonticonsFi }),
            },

            IconName {
                name: "FaFonticons".to_string(),
                icon: rsx!(Icon { icon: FaFonticons }),
            },

            IconName {
                name: "FaFortAwesomeAlt".to_string(),
                icon: rsx!(Icon { icon: FaFortAwesomeAlt }),
            },

            IconName {
                name: "FaFortAwesome".to_string(),
                icon: rsx!(Icon { icon: FaFortAwesome }),
            },

            IconName {
                name: "FaForumbee".to_string(),
                icon: rsx!(Icon { icon: FaForumbee }),
            },

            IconName {
                name: "FaFoursquare".to_string(),
                icon: rsx!(Icon { icon: FaFoursquare }),
            },

            IconName {
                name: "FaFreeCodeCamp".to_string(),
                icon: rsx!(Icon { icon: FaFreeCodeCamp }),
            },

            IconName {
                name: "FaFreebsd".to_string(),
                icon: rsx!(Icon { icon: FaFreebsd }),
            },

            IconName {
                name: "FaFulcrum".to_string(),
                icon: rsx!(Icon { icon: FaFulcrum }),
            },

            IconName {
                name: "FaGalacticRepublic".to_string(),
                icon: rsx!(Icon { icon: FaGalacticRepublic }),
            },

            IconName {
                name: "FaGalacticSenate".to_string(),
                icon: rsx!(Icon { icon: FaGalacticSenate }),
            },

            IconName {
                name: "FaGetPocket".to_string(),
                icon: rsx!(Icon { icon: FaGetPocket }),
            },

            IconName {
                name: "FaGgCircle".to_string(),
                icon: rsx!(Icon { icon: FaGgCircle }),
            },

            IconName {
                name: "FaGg".to_string(),
                icon: rsx!(Icon { icon: FaGg }),
            },

            IconName {
                name: "FaGitAlt".to_string(),
                icon: rsx!(Icon { icon: FaGitAlt }),
            },

            IconName {
                name: "FaGitSquare".to_string(),
                icon: rsx!(Icon { icon: FaGitSquare }),
            },

            IconName {
                name: "FaGit".to_string(),
                icon: rsx!(Icon { icon: FaGit }),
            },

            IconName {
                name: "FaGithubAlt".to_string(),
                icon: rsx!(Icon { icon: FaGithubAlt }),
            },

            IconName {
                name: "FaGithubSquare".to_string(),
                icon: rsx!(Icon { icon: FaGithubSquare }),
            },

            IconName {
                name: "FaGithub".to_string(),
                icon: rsx!(Icon { icon: FaGithub }),
            },

            IconName {
                name: "FaGitkraken".to_string(),
                icon: rsx!(Icon { icon: FaGitkraken }),
            },

            IconName {
                name: "FaGitlab".to_string(),
                icon: rsx!(Icon { icon: FaGitlab }),
            },

            IconName {
                name: "FaGitter".to_string(),
                icon: rsx!(Icon { icon: FaGitter }),
            },

            IconName {
                name: "FaGlideG".to_string(),
                icon: rsx!(Icon { icon: FaGlideG }),
            },

            IconName {
                name: "FaGlide".to_string(),
                icon: rsx!(Icon { icon: FaGlide }),
            },

            IconName {
                name: "FaGofore".to_string(),
                icon: rsx!(Icon { icon: FaGofore }),
            },

            IconName {
                name: "FaGolang".to_string(),
                icon: rsx!(Icon { icon: FaGolang }),
            },

            IconName {
                name: "FaGoodreadsG".to_string(),
                icon: rsx!(Icon { icon: FaGoodreadsG }),
            },

            IconName {
                name: "FaGoodreads".to_string(),
                icon: rsx!(Icon { icon: FaGoodreads }),
            },

            IconName {
                name: "FaGoogleDrive".to_string(),
                icon: rsx!(Icon { icon: FaGoogleDrive }),
            },

            IconName {
                name: "FaGooglePay".to_string(),
                icon: rsx!(Icon { icon: FaGooglePay }),
            },

            IconName {
                name: "FaGooglePlay".to_string(),
                icon: rsx!(Icon { icon: FaGooglePlay }),
            },

            IconName {
                name: "FaGooglePlusG".to_string(),
                icon: rsx!(Icon { icon: FaGooglePlusG }),
            },

            IconName {
                name: "FaGooglePlusSquare".to_string(),
                icon: rsx!(Icon { icon: FaGooglePlusSquare }),
            },

            IconName {
                name: "FaGooglePlus".to_string(),
                icon: rsx!(Icon { icon: FaGooglePlus }),
            },

            IconName {
                name: "FaGoogleWallet".to_string(),
                icon: rsx!(Icon { icon: FaGoogleWallet }),
            },

            IconName {
                name: "FaGoogle".to_string(),
                icon: rsx!(Icon { icon: FaGoogle }),
            },

            IconName {
                name: "FaGratipay".to_string(),
                icon: rsx!(Icon { icon: FaGratipay }),
            },

            IconName {
                name: "FaGrav".to_string(),
                icon: rsx!(Icon { icon: FaGrav }),
            },

            IconName {
                name: "FaGripfire".to_string(),
                icon: rsx!(Icon { icon: FaGripfire }),
            },

            IconName {
                name: "FaGrunt".to_string(),
                icon: rsx!(Icon { icon: FaGrunt }),
            },

            IconName {
                name: "FaGuilded".to_string(),
                icon: rsx!(Icon { icon: FaGuilded }),
            },

            IconName {
                name: "FaGulp".to_string(),
                icon: rsx!(Icon { icon: FaGulp }),
            },

            IconName {
                name: "FaHackerNewsSquare".to_string(),
                icon: rsx!(Icon { icon: FaHackerNewsSquare }),
            },

            IconName {
                name: "FaHackerNews".to_string(),
                icon: rsx!(Icon { icon: FaHackerNews }),
            },

            IconName {
                name: "FaHackerrank".to_string(),
                icon: rsx!(Icon { icon: FaHackerrank }),
            },

            IconName {
                name: "FaHashnode".to_string(),
                icon: rsx!(Icon { icon: FaHashnode }),
            },

            IconName {
                name: "FaHips".to_string(),
                icon: rsx!(Icon { icon: FaHips }),
            },

            IconName {
                name: "FaHireAHelper".to_string(),
                icon: rsx!(Icon { icon: FaHireAHelper }),
            },

            IconName {
                name: "FaHive".to_string(),
                icon: rsx!(Icon { icon: FaHive }),
            },

            IconName {
                name: "FaHooli".to_string(),
                icon: rsx!(Icon { icon: FaHooli }),
            },

            IconName {
                name: "FaHornbill".to_string(),
                icon: rsx!(Icon { icon: FaHornbill }),
            },

            IconName {
                name: "FaHotjar".to_string(),
                icon: rsx!(Icon { icon: FaHotjar }),
            },

            IconName {
                name: "FaHouzz".to_string(),
                icon: rsx!(Icon { icon: FaHouzz }),
            },

            IconName {
                name: "FaHtml5".to_string(),
                icon: rsx!(Icon { icon: FaHtml5 }),
            },

            IconName {
                name: "FaHubspot".to_string(),
                icon: rsx!(Icon { icon: FaHubspot }),
            },

            IconName {
                name: "FaIdeal".to_string(),
                icon: rsx!(Icon { icon: FaIdeal }),
            },

            IconName {
                name: "FaImdb".to_string(),
                icon: rsx!(Icon { icon: FaImdb }),
            },

            IconName {
                name: "FaInstagramSquare".to_string(),
                icon: rsx!(Icon { icon: FaInstagramSquare }),
            },

            IconName {
                name: "FaInstagram".to_string(),
                icon: rsx!(Icon { icon: FaInstagram }),
            },

            IconName {
                name: "FaInstalod".to_string(),
                icon: rsx!(Icon { icon: FaInstalod }),
            },

            IconName {
                name: "FaIntercom".to_string(),
                icon: rsx!(Icon { icon: FaIntercom }),
            },

            IconName {
                name: "FaInternetExplorer".to_string(),
                icon: rsx!(Icon { icon: FaInternetExplorer }),
            },

            IconName {
                name: "FaInvision".to_string(),
                icon: rsx!(Icon { icon: FaInvision }),
            },

            IconName {
                name: "FaIoxhost".to_string(),
                icon: rsx!(Icon { icon: FaIoxhost }),
            },

            IconName {
                name: "FaItchIo".to_string(),
                icon: rsx!(Icon { icon: FaItchIo }),
            },

            IconName {
                name: "FaItunesNote".to_string(),
                icon: rsx!(Icon { icon: FaItunesNote }),
            },

            IconName {
                name: "FaItunes".to_string(),
                icon: rsx!(Icon { icon: FaItunes }),
            },

            IconName {
                name: "FaJava".to_string(),
                icon: rsx!(Icon { icon: FaJava }),
            },

            IconName {
                name: "FaJediOrder".to_string(),
                icon: rsx!(Icon { icon: FaJediOrder }),
            },

            IconName {
                name: "FaJenkins".to_string(),
                icon: rsx!(Icon { icon: FaJenkins }),
            },

            IconName {
                name: "FaJira".to_string(),
                icon: rsx!(Icon { icon: FaJira }),
            },

            IconName {
                name: "FaJoget".to_string(),
                icon: rsx!(Icon { icon: FaJoget }),
            },

            IconName {
                name: "FaJoomla".to_string(),
                icon: rsx!(Icon { icon: FaJoomla }),
            },

            IconName {
                name: "FaJsSquare".to_string(),
                icon: rsx!(Icon { icon: FaJsSquare }),
            },

            IconName {
                name: "FaJs".to_string(),
                icon: rsx!(Icon { icon: FaJs }),
            },

            IconName {
                name: "FaJsfiddle".to_string(),
                icon: rsx!(Icon { icon: FaJsfiddle }),
            },

            IconName {
                name: "FaKaggle".to_string(),
                icon: rsx!(Icon { icon: FaKaggle }),
            },

            IconName {
                name: "FaKeybase".to_string(),
                icon: rsx!(Icon { icon: FaKeybase }),
            },

            IconName {
                name: "FaKeycdn".to_string(),
                icon: rsx!(Icon { icon: FaKeycdn }),
            },

            IconName {
                name: "FaKickstarterK".to_string(),
                icon: rsx!(Icon { icon: FaKickstarterK }),
            },

            IconName {
                name: "FaKickstarter".to_string(),
                icon: rsx!(Icon { icon: FaKickstarter }),
            },

            IconName {
                name: "FaKorvue".to_string(),
                icon: rsx!(Icon { icon: FaKorvue }),
            },

            IconName {
                name: "FaLaravel".to_string(),
                icon: rsx!(Icon { icon: FaLaravel }),
            },

            IconName {
                name: "FaLastfmSquare".to_string(),
                icon: rsx!(Icon { icon: FaLastfmSquare }),
            },

            IconName {
                name: "FaLastfm".to_string(),
                icon: rsx!(Icon { icon: FaLastfm }),
            },

            IconName {
                name: "FaLeanpub".to_string(),
                icon: rsx!(Icon { icon: FaLeanpub }),
            },

            IconName {
                name: "FaLess".to_string(),
                icon: rsx!(Icon { icon: FaLess }),
            },

            IconName {
                name: "FaLine".to_string(),
                icon: rsx!(Icon { icon: FaLine }),
            },

            IconName {
                name: "FaLinkedinIn".to_string(),
                icon: rsx!(Icon { icon: FaLinkedinIn }),
            },

            IconName {
                name: "FaLinkedin".to_string(),
                icon: rsx!(Icon { icon: FaLinkedin }),
            },

            IconName {
                name: "FaLinode".to_string(),
                icon: rsx!(Icon { icon: FaLinode }),
            },

            IconName {
                name: "FaLinux".to_string(),
                icon: rsx!(Icon { icon: FaLinux }),
            },

            IconName {
                name: "FaLyft".to_string(),
                icon: rsx!(Icon { icon: FaLyft }),
            },

            IconName {
                name: "FaMagento".to_string(),
                icon: rsx!(Icon { icon: FaMagento }),
            },

            IconName {
                name: "FaMailchimp".to_string(),
                icon: rsx!(Icon { icon: FaMailchimp }),
            },

            IconName {
                name: "FaMandalorian".to_string(),
                icon: rsx!(Icon { icon: FaMandalorian }),
            },

            IconName {
                name: "FaMarkdown".to_string(),
                icon: rsx!(Icon { icon: FaMarkdown }),
            },

            IconName {
                name: "FaMastodon".to_string(),
                icon: rsx!(Icon { icon: FaMastodon }),
            },

            IconName {
                name: "FaMaxcdn".to_string(),
                icon: rsx!(Icon { icon: FaMaxcdn }),
            },

            IconName {
                name: "FaMdb".to_string(),
                icon: rsx!(Icon { icon: FaMdb }),
            },

            IconName {
                name: "FaMedapps".to_string(),
                icon: rsx!(Icon { icon: FaMedapps }),
            },

            IconName {
                name: "FaMedium".to_string(),
                icon: rsx!(Icon { icon: FaMedium }),
            },

            IconName {
                name: "FaMedrt".to_string(),
                icon: rsx!(Icon { icon: FaMedrt }),
            },

            IconName {
                name: "FaMeetup".to_string(),
                icon: rsx!(Icon { icon: FaMeetup }),
            },

            IconName {
                name: "FaMegaport".to_string(),
                icon: rsx!(Icon { icon: FaMegaport }),
            },

            IconName {
                name: "FaMendeley".to_string(),
                icon: rsx!(Icon { icon: FaMendeley }),
            },

            IconName {
                name: "FaMicroblog".to_string(),
                icon: rsx!(Icon { icon: FaMicroblog }),
            },

            IconName {
                name: "FaMicrosoft".to_string(),
                icon: rsx!(Icon { icon: FaMicrosoft }),
            },

            IconName {
                name: "FaMix".to_string(),
                icon: rsx!(Icon { icon: FaMix }),
            },

            IconName {
                name: "FaMixcloud".to_string(),
                icon: rsx!(Icon { icon: FaMixcloud }),
            },

            IconName {
                name: "FaMixer".to_string(),
                icon: rsx!(Icon { icon: FaMixer }),
            },

            IconName {
                name: "FaMizuni".to_string(),
                icon: rsx!(Icon { icon: FaMizuni }),
            },

            IconName {
                name: "FaModx".to_string(),
                icon: rsx!(Icon { icon: FaModx }),
            },

            IconName {
                name: "FaMonero".to_string(),
                icon: rsx!(Icon { icon: FaMonero }),
            },

            IconName {
                name: "FaNapster".to_string(),
                icon: rsx!(Icon { icon: FaNapster }),
            },

            IconName {
                name: "FaNeos".to_string(),
                icon: rsx!(Icon { icon: FaNeos }),
            },

            IconName {
                name: "FaNfcDirectional".to_string(),
                icon: rsx!(Icon { icon: FaNfcDirectional }),
            },

            IconName {
                name: "FaNfcSymbol".to_string(),
                icon: rsx!(Icon { icon: FaNfcSymbol }),
            },

            IconName {
                name: "FaNimblr".to_string(),
                icon: rsx!(Icon { icon: FaNimblr }),
            },

            IconName {
                name: "FaNodeJs".to_string(),
                icon: rsx!(Icon { icon: FaNodeJs }),
            },

            IconName {
                name: "FaNode".to_string(),
                icon: rsx!(Icon { icon: FaNode }),
            },

            IconName {
                name: "FaNpm".to_string(),
                icon: rsx!(Icon { icon: FaNpm }),
            },

            IconName {
                name: "FaNs8".to_string(),
                icon: rsx!(Icon { icon: FaNs8 }),
            },

            IconName {
                name: "FaNutritionix".to_string(),
                icon: rsx!(Icon { icon: FaNutritionix }),
            },

            IconName {
                name: "FaOctopusDeploy".to_string(),
                icon: rsx!(Icon { icon: FaOctopusDeploy }),
            },

            IconName {
                name: "FaOdnoklassnikiSquare".to_string(),
                icon: rsx!(Icon { icon: FaOdnoklassnikiSquare }),
            },

            IconName {
                name: "FaOdnoklassniki".to_string(),
                icon: rsx!(Icon { icon: FaOdnoklassniki }),
            },

            IconName {
                name: "FaOldRepublic".to_string(),
                icon: rsx!(Icon { icon: FaOldRepublic }),
            },

            IconName {
                name: "FaOpencart".to_string(),
                icon: rsx!(Icon { icon: FaOpencart }),
            },

            IconName {
                name: "FaOpenid".to_string(),
                icon: rsx!(Icon { icon: FaOpenid }),
            },

            IconName {
                name: "FaOpera".to_string(),
                icon: rsx!(Icon { icon: FaOpera }),
            },

            IconName {
                name: "FaOptinMonster".to_string(),
                icon: rsx!(Icon { icon: FaOptinMonster }),
            },

            IconName {
                name: "FaOrcid".to_string(),
                icon: rsx!(Icon { icon: FaOrcid }),
            },

            IconName {
                name: "FaOsi".to_string(),
                icon: rsx!(Icon { icon: FaOsi }),
            },

            IconName {
                name: "FaPadlet".to_string(),
                icon: rsx!(Icon { icon: FaPadlet }),
            },

            IconName {
                name: "FaPage4".to_string(),
                icon: rsx!(Icon { icon: FaPage4 }),
            },

            IconName {
                name: "FaPagelines".to_string(),
                icon: rsx!(Icon { icon: FaPagelines }),
            },

            IconName {
                name: "FaPalfed".to_string(),
                icon: rsx!(Icon { icon: FaPalfed }),
            },

            IconName {
                name: "FaPatreon".to_string(),
                icon: rsx!(Icon { icon: FaPatreon }),
            },

            IconName {
                name: "FaPaypal".to_string(),
                icon: rsx!(Icon { icon: FaPaypal }),
            },

            IconName {
                name: "FaPerbyte".to_string(),
                icon: rsx!(Icon { icon: FaPerbyte }),
            },

            IconName {
                name: "FaPeriscope".to_string(),
                icon: rsx!(Icon { icon: FaPeriscope }),
            },

            IconName {
                name: "FaPhabricator".to_string(),
                icon: rsx!(Icon { icon: FaPhabricator }),
            },

            IconName {
                name: "FaPhoenixFramework".to_string(),
                icon: rsx!(Icon { icon: FaPhoenixFramework }),
            },

            IconName {
                name: "FaPhoenixSquadron".to_string(),
                icon: rsx!(Icon { icon: FaPhoenixSquadron }),
            },

            IconName {
                name: "FaPhp".to_string(),
                icon: rsx!(Icon { icon: FaPhp }),
            },

            IconName {
                name: "FaPiedPiperAlt".to_string(),
                icon: rsx!(Icon { icon: FaPiedPiperAlt }),
            },

            IconName {
                name: "FaPiedPiperHat".to_string(),
                icon: rsx!(Icon { icon: FaPiedPiperHat }),
            },

            IconName {
                name: "FaPiedPiperPp".to_string(),
                icon: rsx!(Icon { icon: FaPiedPiperPp }),
            },

            IconName {
                name: "FaPiedPiperSquare".to_string(),
                icon: rsx!(Icon { icon: FaPiedPiperSquare }),
            },

            IconName {
                name: "FaPiedPiper".to_string(),
                icon: rsx!(Icon { icon: FaPiedPiper }),
            },

            IconName {
                name: "FaPinterestP".to_string(),
                icon: rsx!(Icon { icon: FaPinterestP }),
            },

            IconName {
                name: "FaPinterestSquare".to_string(),
                icon: rsx!(Icon { icon: FaPinterestSquare }),
            },

            IconName {
                name: "FaPinterest".to_string(),
                icon: rsx!(Icon { icon: FaPinterest }),
            },

            IconName {
                name: "FaPix".to_string(),
                icon: rsx!(Icon { icon: FaPix }),
            },

            IconName {
                name: "FaPlaystation".to_string(),
                icon: rsx!(Icon { icon: FaPlaystation }),
            },

            IconName {
                name: "FaProductHunt".to_string(),
                icon: rsx!(Icon { icon: FaProductHunt }),
            },

            IconName {
                name: "FaPushed".to_string(),
                icon: rsx!(Icon { icon: FaPushed }),
            },

            IconName {
                name: "FaPython".to_string(),
                icon: rsx!(Icon { icon: FaPython }),
            },

            IconName {
                name: "FaQq".to_string(),
                icon: rsx!(Icon { icon: FaQq }),
            },

            IconName {
                name: "FaQuinscape".to_string(),
                icon: rsx!(Icon { icon: FaQuinscape }),
            },

            IconName {
                name: "FaQuora".to_string(),
                icon: rsx!(Icon { icon: FaQuora }),
            },

            IconName {
                name: "FaRProject".to_string(),
                icon: rsx!(Icon { icon: FaRProject }),
            },

            IconName {
                name: "FaRaspberryPi".to_string(),
                icon: rsx!(Icon { icon: FaRaspberryPi }),
            },

            IconName {
                name: "FaRavelry".to_string(),
                icon: rsx!(Icon { icon: FaRavelry }),
            },

            IconName {
                name: "FaReact".to_string(),
                icon: rsx!(Icon { icon: FaReact }),
            },

            IconName {
                name: "FaReacteurope".to_string(),
                icon: rsx!(Icon { icon: FaReacteurope }),
            },

            IconName {
                name: "FaReadme".to_string(),
                icon: rsx!(Icon { icon: FaReadme }),
            },

            IconName {
                name: "FaRebel".to_string(),
                icon: rsx!(Icon { icon: FaRebel }),
            },

            IconName {
                name: "FaRedRiver".to_string(),
                icon: rsx!(Icon { icon: FaRedRiver }),
            },

            IconName {
                name: "FaRedditAlien".to_string(),
                icon: rsx!(Icon { icon: FaRedditAlien }),
            },

            IconName {
                name: "FaRedditSquare".to_string(),
                icon: rsx!(Icon { icon: FaRedditSquare }),
            },

            IconName {
                name: "FaReddit".to_string(),
                icon: rsx!(Icon { icon: FaReddit }),
            },

            IconName {
                name: "FaRedhat".to_string(),
                icon: rsx!(Icon { icon: FaRedhat }),
            },

            IconName {
                name: "FaRenren".to_string(),
                icon: rsx!(Icon { icon: FaRenren }),
            },

            IconName {
                name: "FaReplyd".to_string(),
                icon: rsx!(Icon { icon: FaReplyd }),
            },

            IconName {
                name: "FaResearchgate".to_string(),
                icon: rsx!(Icon { icon: FaResearchgate }),
            },

            IconName {
                name: "FaResolving".to_string(),
                icon: rsx!(Icon { icon: FaResolving }),
            },

            IconName {
                name: "FaRev".to_string(),
                icon: rsx!(Icon { icon: FaRev }),
            },

            IconName {
                name: "FaRocketchat".to_string(),
                icon: rsx!(Icon { icon: FaRocketchat }),
            },

            IconName {
                name: "FaRockrms".to_string(),
                icon: rsx!(Icon { icon: FaRockrms }),
            },

            IconName {
                name: "FaRust".to_string(),
                icon: rsx!(Icon { icon: FaRust }),
            },

            IconName {
                name: "FaSafari".to_string(),
                icon: rsx!(Icon { icon: FaSafari }),
            },

            IconName {
                name: "FaSalesforce".to_string(),
                icon: rsx!(Icon { icon: FaSalesforce }),
            },

            IconName {
                name: "FaSass".to_string(),
                icon: rsx!(Icon { icon: FaSass }),
            },

            IconName {
                name: "FaSchlix".to_string(),
                icon: rsx!(Icon { icon: FaSchlix }),
            },

            IconName {
                name: "FaScreenpal".to_string(),
                icon: rsx!(Icon { icon: FaScreenpal }),
            },

            IconName {
                name: "FaScribd".to_string(),
                icon: rsx!(Icon { icon: FaScribd }),
            },

            IconName {
                name: "FaSearchengin".to_string(),
                icon: rsx!(Icon { icon: FaSearchengin }),
            },

            IconName {
                name: "FaSellcast".to_string(),
                icon: rsx!(Icon { icon: FaSellcast }),
            },

            IconName {
                name: "FaSellsy".to_string(),
                icon: rsx!(Icon { icon: FaSellsy }),
            },

            IconName {
                name: "FaServicestack".to_string(),
                icon: rsx!(Icon { icon: FaServicestack }),
            },

            IconName {
                name: "FaShirtsinbulk".to_string(),
                icon: rsx!(Icon { icon: FaShirtsinbulk }),
            },

            IconName {
                name: "FaShopify".to_string(),
                icon: rsx!(Icon { icon: FaShopify }),
            },

            IconName {
                name: "FaShopware".to_string(),
                icon: rsx!(Icon { icon: FaShopware }),
            },

            IconName {
                name: "FaSimplybuilt".to_string(),
                icon: rsx!(Icon { icon: FaSimplybuilt }),
            },

            IconName {
                name: "FaSistrix".to_string(),
                icon: rsx!(Icon { icon: FaSistrix }),
            },

            IconName {
                name: "FaSith".to_string(),
                icon: rsx!(Icon { icon: FaSith }),
            },

            IconName {
                name: "FaSitrox".to_string(),
                icon: rsx!(Icon { icon: FaSitrox }),
            },

            IconName {
                name: "FaSketch".to_string(),
                icon: rsx!(Icon { icon: FaSketch }),
            },

            IconName {
                name: "FaSkyatlas".to_string(),
                icon: rsx!(Icon { icon: FaSkyatlas }),
            },

            IconName {
                name: "FaSkype".to_string(),
                icon: rsx!(Icon { icon: FaSkype }),
            },

            IconName {
                name: "FaSlack".to_string(),
                icon: rsx!(Icon { icon: FaSlack }),
            },

            IconName {
                name: "FaSlideshare".to_string(),
                icon: rsx!(Icon { icon: FaSlideshare }),
            },

            IconName {
                name: "FaSnapchatSquare".to_string(),
                icon: rsx!(Icon { icon: FaSnapchatSquare }),
            },

            IconName {
                name: "FaSnapchat".to_string(),
                icon: rsx!(Icon { icon: FaSnapchat }),
            },

            IconName {
                name: "FaSoundcloud".to_string(),
                icon: rsx!(Icon { icon: FaSoundcloud }),
            },

            IconName {
                name: "FaSourcetree".to_string(),
                icon: rsx!(Icon { icon: FaSourcetree }),
            },

            IconName {
                name: "FaSpeakap".to_string(),
                icon: rsx!(Icon { icon: FaSpeakap }),
            },

            IconName {
                name: "FaSpeakerDeck".to_string(),
                icon: rsx!(Icon { icon: FaSpeakerDeck }),
            },

            IconName {
                name: "FaSpotify".to_string(),
                icon: rsx!(Icon { icon: FaSpotify }),
            },

            IconName {
                name: "FaSquareFontAwesomeStroke".to_string(),
                icon: rsx!(Icon { icon: FaSquareFontAwesomeStroke }),
            },

            IconName {
                name: "FaSquareFontAwesome".to_string(),
                icon: rsx!(Icon { icon: FaSquareFontAwesome }),
            },

            IconName {
                name: "FaSquarespace".to_string(),
                icon: rsx!(Icon { icon: FaSquarespace }),
            },

            IconName {
                name: "FaStackExchange".to_string(),
                icon: rsx!(Icon { icon: FaStackExchange }),
            },

            IconName {
                name: "FaStackOverflow".to_string(),
                icon: rsx!(Icon { icon: FaStackOverflow }),
            },

            IconName {
                name: "FaStackpath".to_string(),
                icon: rsx!(Icon { icon: FaStackpath }),
            },

            IconName {
                name: "FaStaylinked".to_string(),
                icon: rsx!(Icon { icon: FaStaylinked }),
            },

            IconName {
                name: "FaSteamSquare".to_string(),
                icon: rsx!(Icon { icon: FaSteamSquare }),
            },

            IconName {
                name: "FaSteamSymbol".to_string(),
                icon: rsx!(Icon { icon: FaSteamSymbol }),
            },

            IconName {
                name: "FaSteam".to_string(),
                icon: rsx!(Icon { icon: FaSteam }),
            },

            IconName {
                name: "FaStickerMule".to_string(),
                icon: rsx!(Icon { icon: FaStickerMule }),
            },

            IconName {
                name: "FaStrava".to_string(),
                icon: rsx!(Icon { icon: FaStrava }),
            },

            IconName {
                name: "FaStripeS".to_string(),
                icon: rsx!(Icon { icon: FaStripeS }),
            },

            IconName {
                name: "FaStripe".to_string(),
                icon: rsx!(Icon { icon: FaStripe }),
            },

            IconName {
                name: "FaStudiovinari".to_string(),
                icon: rsx!(Icon { icon: FaStudiovinari }),
            },

            IconName {
                name: "FaStumbleuponCircle".to_string(),
                icon: rsx!(Icon { icon: FaStumbleuponCircle }),
            },

            IconName {
                name: "FaStumbleupon".to_string(),
                icon: rsx!(Icon { icon: FaStumbleupon }),
            },

            IconName {
                name: "FaSuperpowers".to_string(),
                icon: rsx!(Icon { icon: FaSuperpowers }),
            },

            IconName {
                name: "FaSupple".to_string(),
                icon: rsx!(Icon { icon: FaSupple }),
            },

            IconName {
                name: "FaSuse".to_string(),
                icon: rsx!(Icon { icon: FaSuse }),
            },

            IconName {
                name: "FaSwift".to_string(),
                icon: rsx!(Icon { icon: FaSwift }),
            },

            IconName {
                name: "FaSymfony".to_string(),
                icon: rsx!(Icon { icon: FaSymfony }),
            },

            IconName {
                name: "FaTeamspeak".to_string(),
                icon: rsx!(Icon { icon: FaTeamspeak }),
            },

            IconName {
                name: "FaTelegram".to_string(),
                icon: rsx!(Icon { icon: FaTelegram }),
            },

            IconName {
                name: "FaTencentWeibo".to_string(),
                icon: rsx!(Icon { icon: FaTencentWeibo }),
            },

            IconName {
                name: "FaTheRedYeti".to_string(),
                icon: rsx!(Icon { icon: FaTheRedYeti }),
            },

            IconName {
                name: "FaThemeco".to_string(),
                icon: rsx!(Icon { icon: FaThemeco }),
            },

            IconName {
                name: "FaThemeisle".to_string(),
                icon: rsx!(Icon { icon: FaThemeisle }),
            },

            IconName {
                name: "FaThinkPeaks".to_string(),
                icon: rsx!(Icon { icon: FaThinkPeaks }),
            },

            IconName {
                name: "FaTiktok".to_string(),
                icon: rsx!(Icon { icon: FaTiktok }),
            },

            IconName {
                name: "FaTradeFederation".to_string(),
                icon: rsx!(Icon { icon: FaTradeFederation }),
            },

            IconName {
                name: "FaTrello".to_string(),
                icon: rsx!(Icon { icon: FaTrello }),
            },

            IconName {
                name: "FaTumblrSquare".to_string(),
                icon: rsx!(Icon { icon: FaTumblrSquare }),
            },

            IconName {
                name: "FaTumblr".to_string(),
                icon: rsx!(Icon { icon: FaTumblr }),
            },

            IconName {
                name: "FaTwitch".to_string(),
                icon: rsx!(Icon { icon: FaTwitch }),
            },

            IconName {
                name: "FaTwitterSquare".to_string(),
                icon: rsx!(Icon { icon: FaTwitterSquare }),
            },

            IconName {
                name: "FaTwitter".to_string(),
                icon: rsx!(Icon { icon: FaTwitter }),
            },

            IconName {
                name: "FaTypo3".to_string(),
                icon: rsx!(Icon { icon: FaTypo3 }),
            },

            IconName {
                name: "FaUber".to_string(),
                icon: rsx!(Icon { icon: FaUber }),
            },

            IconName {
                name: "FaUbuntu".to_string(),
                icon: rsx!(Icon { icon: FaUbuntu }),
            },

            IconName {
                name: "FaUikit".to_string(),
                icon: rsx!(Icon { icon: FaUikit }),
            },

            IconName {
                name: "FaUmbraco".to_string(),
                icon: rsx!(Icon { icon: FaUmbraco }),
            },

            IconName {
                name: "FaUncharted".to_string(),
                icon: rsx!(Icon { icon: FaUncharted }),
            },

            IconName {
                name: "FaUniregistry".to_string(),
                icon: rsx!(Icon { icon: FaUniregistry }),
            },

            IconName {
                name: "FaUnity".to_string(),
                icon: rsx!(Icon { icon: FaUnity }),
            },

            IconName {
                name: "FaUnsplash".to_string(),
                icon: rsx!(Icon { icon: FaUnsplash }),
            },

            IconName {
                name: "FaUntappd".to_string(),
                icon: rsx!(Icon { icon: FaUntappd }),
            },

            IconName {
                name: "FaUps".to_string(),
                icon: rsx!(Icon { icon: FaUps }),
            },

            IconName {
                name: "FaUsb".to_string(),
                icon: rsx!(Icon { icon: FaUsb }),
            },

            IconName {
                name: "FaUsps".to_string(),
                icon: rsx!(Icon { icon: FaUsps }),
            },

            IconName {
                name: "FaUssunnah".to_string(),
                icon: rsx!(Icon { icon: FaUssunnah }),
            },

            IconName {
                name: "FaVaadin".to_string(),
                icon: rsx!(Icon { icon: FaVaadin }),
            },

            IconName {
                name: "FaViacoin".to_string(),
                icon: rsx!(Icon { icon: FaViacoin }),
            },

            IconName {
                name: "FaViadeoSquare".to_string(),
                icon: rsx!(Icon { icon: FaViadeoSquare }),
            },

            IconName {
                name: "FaViadeo".to_string(),
                icon: rsx!(Icon { icon: FaViadeo }),
            },

            IconName {
                name: "FaViber".to_string(),
                icon: rsx!(Icon { icon: FaViber }),
            },

            IconName {
                name: "FaVimeoSquare".to_string(),
                icon: rsx!(Icon { icon: FaVimeoSquare }),
            },

            IconName {
                name: "FaVimeoV".to_string(),
                icon: rsx!(Icon { icon: FaVimeoV }),
            },

            IconName {
                name: "FaVimeo".to_string(),
                icon: rsx!(Icon { icon: FaVimeo }),
            },

            IconName {
                name: "FaVine".to_string(),
                icon: rsx!(Icon { icon: FaVine }),
            },

            IconName {
                name: "FaVk".to_string(),
                icon: rsx!(Icon { icon: FaVk }),
            },

            IconName {
                name: "FaVnv".to_string(),
                icon: rsx!(Icon { icon: FaVnv }),
            },

            IconName {
                name: "FaVuejs".to_string(),
                icon: rsx!(Icon { icon: FaVuejs }),
            },

            IconName {
                name: "FaWatchmanMonitoring".to_string(),
                icon: rsx!(Icon { icon: FaWatchmanMonitoring }),
            },

            IconName {
                name: "FaWaze".to_string(),
                icon: rsx!(Icon { icon: FaWaze }),
            },

            IconName {
                name: "FaWeebly".to_string(),
                icon: rsx!(Icon { icon: FaWeebly }),
            },

            IconName {
                name: "FaWeibo".to_string(),
                icon: rsx!(Icon { icon: FaWeibo }),
            },

            IconName {
                name: "FaWeixin".to_string(),
                icon: rsx!(Icon { icon: FaWeixin }),
            },

            IconName {
                name: "FaWhatsappSquare".to_string(),
                icon: rsx!(Icon { icon: FaWhatsappSquare }),
            },

            IconName {
                name: "FaWhatsapp".to_string(),
                icon: rsx!(Icon { icon: FaWhatsapp }),
            },

            IconName {
                name: "FaWhmcs".to_string(),
                icon: rsx!(Icon { icon: FaWhmcs }),
            },

            IconName {
                name: "FaWikipediaW".to_string(),
                icon: rsx!(Icon { icon: FaWikipediaW }),
            },

            IconName {
                name: "FaWindows".to_string(),
                icon: rsx!(Icon { icon: FaWindows }),
            },

            IconName {
                name: "FaWirsindhandwerk".to_string(),
                icon: rsx!(Icon { icon: FaWirsindhandwerk }),
            },

            IconName {
                name: "FaWix".to_string(),
                icon: rsx!(Icon { icon: FaWix }),
            },

            IconName {
                name: "FaWizardsOfTheCoast".to_string(),
                icon: rsx!(Icon { icon: FaWizardsOfTheCoast }),
            },

            IconName {
                name: "FaWodu".to_string(),
                icon: rsx!(Icon { icon: FaWodu }),
            },

            IconName {
                name: "FaWolfPackBattalion".to_string(),
                icon: rsx!(Icon { icon: FaWolfPackBattalion }),
            },

            IconName {
                name: "FaWordpressSimple".to_string(),
                icon: rsx!(Icon { icon: FaWordpressSimple }),
            },

            IconName {
                name: "FaWordpress".to_string(),
                icon: rsx!(Icon { icon: FaWordpress }),
            },

            IconName {
                name: "FaWpbeginner".to_string(),
                icon: rsx!(Icon { icon: FaWpbeginner }),
            },

            IconName {
                name: "FaWpexplorer".to_string(),
                icon: rsx!(Icon { icon: FaWpexplorer }),
            },

            IconName {
                name: "FaWpforms".to_string(),
                icon: rsx!(Icon { icon: FaWpforms }),
            },

            IconName {
                name: "FaWpressr".to_string(),
                icon: rsx!(Icon { icon: FaWpressr }),
            },

            IconName {
                name: "FaXbox".to_string(),
                icon: rsx!(Icon { icon: FaXbox }),
            },

            IconName {
                name: "FaXingSquare".to_string(),
                icon: rsx!(Icon { icon: FaXingSquare }),
            },

            IconName {
                name: "FaXing".to_string(),
                icon: rsx!(Icon { icon: FaXing }),
            },

            IconName {
                name: "FaYCombinator".to_string(),
                icon: rsx!(Icon { icon: FaYCombinator }),
            },

            IconName {
                name: "FaYahoo".to_string(),
                icon: rsx!(Icon { icon: FaYahoo }),
            },

            IconName {
                name: "FaYammer".to_string(),
                icon: rsx!(Icon { icon: FaYammer }),
            },

            IconName {
                name: "FaYandexInternational".to_string(),
                icon: rsx!(Icon { icon: FaYandexInternational }),
            },

            IconName {
                name: "FaYandex".to_string(),
                icon: rsx!(Icon { icon: FaYandex }),
            },

            IconName {
                name: "FaYarn".to_string(),
                icon: rsx!(Icon { icon: FaYarn }),
            },

            IconName {
                name: "FaYelp".to_string(),
                icon: rsx!(Icon { icon: FaYelp }),
            },

            IconName {
                name: "FaYoast".to_string(),
                icon: rsx!(Icon { icon: FaYoast }),
            },

            IconName {
                name: "FaYoutubeSquare".to_string(),
                icon: rsx!(Icon { icon: FaYoutubeSquare }),
            },

            IconName {
                name: "FaYoutube".to_string(),
                icon: rsx!(Icon { icon: FaYoutube }),
            },

            IconName {
                name: "FaZhihu".to_string(),
                icon: rsx!(Icon { icon: FaZhihu }),
            }
        ],
    }
}
