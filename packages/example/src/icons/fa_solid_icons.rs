
use crate::{IconSet, IconName};
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::*;
use dioxus_free_icons::Icon;


pub fn get_icon_set() -> IconSet{
    IconSet {
        code: "fa_solid_icons".to_string(),
        name: "Font Awesome".to_string(),
        url: "https://fontawesome.com/".to_string(),
        license: "CC BY 4.0 License".to_string(),
        license_url: "https://creativecommons.org/licenses/by/4.0/".to_string(),
        version: "6.1.1".to_string(),
        source_url: "https://github.com/FortAwesome/Font-Awesome/tree/6.1.1".to_string(),
        icons: vec![

            IconName {
                name: "Fa0".to_string(),
                icon: rsx!(Icon { icon: Fa0 }),
            },

            IconName {
                name: "Fa1".to_string(),
                icon: rsx!(Icon { icon: Fa1 }),
            },

            IconName {
                name: "Fa2".to_string(),
                icon: rsx!(Icon { icon: Fa2 }),
            },

            IconName {
                name: "Fa3".to_string(),
                icon: rsx!(Icon { icon: Fa3 }),
            },

            IconName {
                name: "Fa4".to_string(),
                icon: rsx!(Icon { icon: Fa4 }),
            },

            IconName {
                name: "Fa5".to_string(),
                icon: rsx!(Icon { icon: Fa5 }),
            },

            IconName {
                name: "Fa6".to_string(),
                icon: rsx!(Icon { icon: Fa6 }),
            },

            IconName {
                name: "Fa7".to_string(),
                icon: rsx!(Icon { icon: Fa7 }),
            },

            IconName {
                name: "Fa8".to_string(),
                icon: rsx!(Icon { icon: Fa8 }),
            },

            IconName {
                name: "Fa9".to_string(),
                icon: rsx!(Icon { icon: Fa9 }),
            },

            IconName {
                name: "FaA".to_string(),
                icon: rsx!(Icon { icon: FaA }),
            },

            IconName {
                name: "FaAddressBook".to_string(),
                icon: rsx!(Icon { icon: FaAddressBook }),
            },

            IconName {
                name: "FaAddressCard".to_string(),
                icon: rsx!(Icon { icon: FaAddressCard }),
            },

            IconName {
                name: "FaAlignCenter".to_string(),
                icon: rsx!(Icon { icon: FaAlignCenter }),
            },

            IconName {
                name: "FaAlignJustify".to_string(),
                icon: rsx!(Icon { icon: FaAlignJustify }),
            },

            IconName {
                name: "FaAlignLeft".to_string(),
                icon: rsx!(Icon { icon: FaAlignLeft }),
            },

            IconName {
                name: "FaAlignRight".to_string(),
                icon: rsx!(Icon { icon: FaAlignRight }),
            },

            IconName {
                name: "FaAnchorCircleCheck".to_string(),
                icon: rsx!(Icon { icon: FaAnchorCircleCheck }),
            },

            IconName {
                name: "FaAnchorCircleExclamation".to_string(),
                icon: rsx!(Icon { icon: FaAnchorCircleExclamation }),
            },

            IconName {
                name: "FaAnchorCircleXmark".to_string(),
                icon: rsx!(Icon { icon: FaAnchorCircleXmark }),
            },

            IconName {
                name: "FaAnchorLock".to_string(),
                icon: rsx!(Icon { icon: FaAnchorLock }),
            },

            IconName {
                name: "FaAnchor".to_string(),
                icon: rsx!(Icon { icon: FaAnchor }),
            },

            IconName {
                name: "FaAngleDown".to_string(),
                icon: rsx!(Icon { icon: FaAngleDown }),
            },

            IconName {
                name: "FaAngleLeft".to_string(),
                icon: rsx!(Icon { icon: FaAngleLeft }),
            },

            IconName {
                name: "FaAngleRight".to_string(),
                icon: rsx!(Icon { icon: FaAngleRight }),
            },

            IconName {
                name: "FaAngleUp".to_string(),
                icon: rsx!(Icon { icon: FaAngleUp }),
            },

            IconName {
                name: "FaAnglesDown".to_string(),
                icon: rsx!(Icon { icon: FaAnglesDown }),
            },

            IconName {
                name: "FaAnglesLeft".to_string(),
                icon: rsx!(Icon { icon: FaAnglesLeft }),
            },

            IconName {
                name: "FaAnglesRight".to_string(),
                icon: rsx!(Icon { icon: FaAnglesRight }),
            },

            IconName {
                name: "FaAnglesUp".to_string(),
                icon: rsx!(Icon { icon: FaAnglesUp }),
            },

            IconName {
                name: "FaAnkh".to_string(),
                icon: rsx!(Icon { icon: FaAnkh }),
            },

            IconName {
                name: "FaAppleWhole".to_string(),
                icon: rsx!(Icon { icon: FaAppleWhole }),
            },

            IconName {
                name: "FaArchway".to_string(),
                icon: rsx!(Icon { icon: FaArchway }),
            },

            IconName {
                name: "FaArrowDown19".to_string(),
                icon: rsx!(Icon { icon: FaArrowDown19 }),
            },

            IconName {
                name: "FaArrowDown91".to_string(),
                icon: rsx!(Icon { icon: FaArrowDown91 }),
            },

            IconName {
                name: "FaArrowDownAZ".to_string(),
                icon: rsx!(Icon { icon: FaArrowDownAZ }),
            },

            IconName {
                name: "FaArrowDownLong".to_string(),
                icon: rsx!(Icon { icon: FaArrowDownLong }),
            },

            IconName {
                name: "FaArrowDownShortWide".to_string(),
                icon: rsx!(Icon { icon: FaArrowDownShortWide }),
            },

            IconName {
                name: "FaArrowDownUpAcrossLine".to_string(),
                icon: rsx!(Icon { icon: FaArrowDownUpAcrossLine }),
            },

            IconName {
                name: "FaArrowDownUpLock".to_string(),
                icon: rsx!(Icon { icon: FaArrowDownUpLock }),
            },

            IconName {
                name: "FaArrowDownWideShort".to_string(),
                icon: rsx!(Icon { icon: FaArrowDownWideShort }),
            },

            IconName {
                name: "FaArrowDownZA".to_string(),
                icon: rsx!(Icon { icon: FaArrowDownZA }),
            },

            IconName {
                name: "FaArrowDown".to_string(),
                icon: rsx!(Icon { icon: FaArrowDown }),
            },

            IconName {
                name: "FaArrowLeftLong".to_string(),
                icon: rsx!(Icon { icon: FaArrowLeftLong }),
            },

            IconName {
                name: "FaArrowLeft".to_string(),
                icon: rsx!(Icon { icon: FaArrowLeft }),
            },

            IconName {
                name: "FaArrowPointer".to_string(),
                icon: rsx!(Icon { icon: FaArrowPointer }),
            },

            IconName {
                name: "FaArrowRightArrowLeft".to_string(),
                icon: rsx!(Icon { icon: FaArrowRightArrowLeft }),
            },

            IconName {
                name: "FaArrowRightFromBracket".to_string(),
                icon: rsx!(Icon { icon: FaArrowRightFromBracket }),
            },

            IconName {
                name: "FaArrowRightLong".to_string(),
                icon: rsx!(Icon { icon: FaArrowRightLong }),
            },

            IconName {
                name: "FaArrowRightToBracket".to_string(),
                icon: rsx!(Icon { icon: FaArrowRightToBracket }),
            },

            IconName {
                name: "FaArrowRightToCity".to_string(),
                icon: rsx!(Icon { icon: FaArrowRightToCity }),
            },

            IconName {
                name: "FaArrowRight".to_string(),
                icon: rsx!(Icon { icon: FaArrowRight }),
            },

            IconName {
                name: "FaArrowRotateLeft".to_string(),
                icon: rsx!(Icon { icon: FaArrowRotateLeft }),
            },

            IconName {
                name: "FaArrowRotateRight".to_string(),
                icon: rsx!(Icon { icon: FaArrowRotateRight }),
            },

            IconName {
                name: "FaArrowTrendDown".to_string(),
                icon: rsx!(Icon { icon: FaArrowTrendDown }),
            },

            IconName {
                name: "FaArrowTrendUp".to_string(),
                icon: rsx!(Icon { icon: FaArrowTrendUp }),
            },

            IconName {
                name: "FaArrowTurnDown".to_string(),
                icon: rsx!(Icon { icon: FaArrowTurnDown }),
            },

            IconName {
                name: "FaArrowTurnUp".to_string(),
                icon: rsx!(Icon { icon: FaArrowTurnUp }),
            },

            IconName {
                name: "FaArrowUp19".to_string(),
                icon: rsx!(Icon { icon: FaArrowUp19 }),
            },

            IconName {
                name: "FaArrowUp91".to_string(),
                icon: rsx!(Icon { icon: FaArrowUp91 }),
            },

            IconName {
                name: "FaArrowUpAZ".to_string(),
                icon: rsx!(Icon { icon: FaArrowUpAZ }),
            },

            IconName {
                name: "FaArrowUpFromBracket".to_string(),
                icon: rsx!(Icon { icon: FaArrowUpFromBracket }),
            },

            IconName {
                name: "FaArrowUpFromGroundWater".to_string(),
                icon: rsx!(Icon { icon: FaArrowUpFromGroundWater }),
            },

            IconName {
                name: "FaArrowUpFromWaterPump".to_string(),
                icon: rsx!(Icon { icon: FaArrowUpFromWaterPump }),
            },

            IconName {
                name: "FaArrowUpLong".to_string(),
                icon: rsx!(Icon { icon: FaArrowUpLong }),
            },

            IconName {
                name: "FaArrowUpRightDots".to_string(),
                icon: rsx!(Icon { icon: FaArrowUpRightDots }),
            },

            IconName {
                name: "FaArrowUpRightFromSquare".to_string(),
                icon: rsx!(Icon { icon: FaArrowUpRightFromSquare }),
            },

            IconName {
                name: "FaArrowUpShortWide".to_string(),
                icon: rsx!(Icon { icon: FaArrowUpShortWide }),
            },

            IconName {
                name: "FaArrowUpWideShort".to_string(),
                icon: rsx!(Icon { icon: FaArrowUpWideShort }),
            },

            IconName {
                name: "FaArrowUpZA".to_string(),
                icon: rsx!(Icon { icon: FaArrowUpZA }),
            },

            IconName {
                name: "FaArrowUp".to_string(),
                icon: rsx!(Icon { icon: FaArrowUp }),
            },

            IconName {
                name: "FaArrowsDownToLine".to_string(),
                icon: rsx!(Icon { icon: FaArrowsDownToLine }),
            },

            IconName {
                name: "FaArrowsDownToPeople".to_string(),
                icon: rsx!(Icon { icon: FaArrowsDownToPeople }),
            },

            IconName {
                name: "FaArrowsLeftRightToLine".to_string(),
                icon: rsx!(Icon { icon: FaArrowsLeftRightToLine }),
            },

            IconName {
                name: "FaArrowsLeftRight".to_string(),
                icon: rsx!(Icon { icon: FaArrowsLeftRight }),
            },

            IconName {
                name: "FaArrowsRotate".to_string(),
                icon: rsx!(Icon { icon: FaArrowsRotate }),
            },

            IconName {
                name: "FaArrowsSpin".to_string(),
                icon: rsx!(Icon { icon: FaArrowsSpin }),
            },

            IconName {
                name: "FaArrowsSplitUpAndLeft".to_string(),
                icon: rsx!(Icon { icon: FaArrowsSplitUpAndLeft }),
            },

            IconName {
                name: "FaArrowsToCircle".to_string(),
                icon: rsx!(Icon { icon: FaArrowsToCircle }),
            },

            IconName {
                name: "FaArrowsToDot".to_string(),
                icon: rsx!(Icon { icon: FaArrowsToDot }),
            },

            IconName {
                name: "FaArrowsToEye".to_string(),
                icon: rsx!(Icon { icon: FaArrowsToEye }),
            },

            IconName {
                name: "FaArrowsTurnRight".to_string(),
                icon: rsx!(Icon { icon: FaArrowsTurnRight }),
            },

            IconName {
                name: "FaArrowsTurnToDots".to_string(),
                icon: rsx!(Icon { icon: FaArrowsTurnToDots }),
            },

            IconName {
                name: "FaArrowsUpDownLeftRight".to_string(),
                icon: rsx!(Icon { icon: FaArrowsUpDownLeftRight }),
            },

            IconName {
                name: "FaArrowsUpDown".to_string(),
                icon: rsx!(Icon { icon: FaArrowsUpDown }),
            },

            IconName {
                name: "FaArrowsUpToLine".to_string(),
                icon: rsx!(Icon { icon: FaArrowsUpToLine }),
            },

            IconName {
                name: "FaAsterisk".to_string(),
                icon: rsx!(Icon { icon: FaAsterisk }),
            },

            IconName {
                name: "FaAt".to_string(),
                icon: rsx!(Icon { icon: FaAt }),
            },

            IconName {
                name: "FaAtom".to_string(),
                icon: rsx!(Icon { icon: FaAtom }),
            },

            IconName {
                name: "FaAudioDescription".to_string(),
                icon: rsx!(Icon { icon: FaAudioDescription }),
            },

            IconName {
                name: "FaAustralSign".to_string(),
                icon: rsx!(Icon { icon: FaAustralSign }),
            },

            IconName {
                name: "FaAward".to_string(),
                icon: rsx!(Icon { icon: FaAward }),
            },

            IconName {
                name: "FaB".to_string(),
                icon: rsx!(Icon { icon: FaB }),
            },

            IconName {
                name: "FaBabyCarriage".to_string(),
                icon: rsx!(Icon { icon: FaBabyCarriage }),
            },

            IconName {
                name: "FaBaby".to_string(),
                icon: rsx!(Icon { icon: FaBaby }),
            },

            IconName {
                name: "FaBackwardFast".to_string(),
                icon: rsx!(Icon { icon: FaBackwardFast }),
            },

            IconName {
                name: "FaBackwardStep".to_string(),
                icon: rsx!(Icon { icon: FaBackwardStep }),
            },

            IconName {
                name: "FaBackward".to_string(),
                icon: rsx!(Icon { icon: FaBackward }),
            },

            IconName {
                name: "FaBacon".to_string(),
                icon: rsx!(Icon { icon: FaBacon }),
            },

            IconName {
                name: "FaBacteria".to_string(),
                icon: rsx!(Icon { icon: FaBacteria }),
            },

            IconName {
                name: "FaBacterium".to_string(),
                icon: rsx!(Icon { icon: FaBacterium }),
            },

            IconName {
                name: "FaBagShopping".to_string(),
                icon: rsx!(Icon { icon: FaBagShopping }),
            },

            IconName {
                name: "FaBahai".to_string(),
                icon: rsx!(Icon { icon: FaBahai }),
            },

            IconName {
                name: "FaBahtSign".to_string(),
                icon: rsx!(Icon { icon: FaBahtSign }),
            },

            IconName {
                name: "FaBanSmoking".to_string(),
                icon: rsx!(Icon { icon: FaBanSmoking }),
            },

            IconName {
                name: "FaBan".to_string(),
                icon: rsx!(Icon { icon: FaBan }),
            },

            IconName {
                name: "FaBandage".to_string(),
                icon: rsx!(Icon { icon: FaBandage }),
            },

            IconName {
                name: "FaBarcode".to_string(),
                icon: rsx!(Icon { icon: FaBarcode }),
            },

            IconName {
                name: "FaBarsProgress".to_string(),
                icon: rsx!(Icon { icon: FaBarsProgress }),
            },

            IconName {
                name: "FaBarsStaggered".to_string(),
                icon: rsx!(Icon { icon: FaBarsStaggered }),
            },

            IconName {
                name: "FaBars".to_string(),
                icon: rsx!(Icon { icon: FaBars }),
            },

            IconName {
                name: "FaBaseballBatBall".to_string(),
                icon: rsx!(Icon { icon: FaBaseballBatBall }),
            },

            IconName {
                name: "FaBaseball".to_string(),
                icon: rsx!(Icon { icon: FaBaseball }),
            },

            IconName {
                name: "FaBasketShopping".to_string(),
                icon: rsx!(Icon { icon: FaBasketShopping }),
            },

            IconName {
                name: "FaBasketball".to_string(),
                icon: rsx!(Icon { icon: FaBasketball }),
            },

            IconName {
                name: "FaBath".to_string(),
                icon: rsx!(Icon { icon: FaBath }),
            },

            IconName {
                name: "FaBatteryEmpty".to_string(),
                icon: rsx!(Icon { icon: FaBatteryEmpty }),
            },

            IconName {
                name: "FaBatteryFull".to_string(),
                icon: rsx!(Icon { icon: FaBatteryFull }),
            },

            IconName {
                name: "FaBatteryHalf".to_string(),
                icon: rsx!(Icon { icon: FaBatteryHalf }),
            },

            IconName {
                name: "FaBatteryQuarter".to_string(),
                icon: rsx!(Icon { icon: FaBatteryQuarter }),
            },

            IconName {
                name: "FaBatteryThreeQuarters".to_string(),
                icon: rsx!(Icon { icon: FaBatteryThreeQuarters }),
            },

            IconName {
                name: "FaBedPulse".to_string(),
                icon: rsx!(Icon { icon: FaBedPulse }),
            },

            IconName {
                name: "FaBed".to_string(),
                icon: rsx!(Icon { icon: FaBed }),
            },

            IconName {
                name: "FaBeerMugEmpty".to_string(),
                icon: rsx!(Icon { icon: FaBeerMugEmpty }),
            },

            IconName {
                name: "FaBellConcierge".to_string(),
                icon: rsx!(Icon { icon: FaBellConcierge }),
            },

            IconName {
                name: "FaBellSlash".to_string(),
                icon: rsx!(Icon { icon: FaBellSlash }),
            },

            IconName {
                name: "FaBell".to_string(),
                icon: rsx!(Icon { icon: FaBell }),
            },

            IconName {
                name: "FaBezierCurve".to_string(),
                icon: rsx!(Icon { icon: FaBezierCurve }),
            },

            IconName {
                name: "FaBicycle".to_string(),
                icon: rsx!(Icon { icon: FaBicycle }),
            },

            IconName {
                name: "FaBinoculars".to_string(),
                icon: rsx!(Icon { icon: FaBinoculars }),
            },

            IconName {
                name: "FaBiohazard".to_string(),
                icon: rsx!(Icon { icon: FaBiohazard }),
            },

            IconName {
                name: "FaBitcoinSign".to_string(),
                icon: rsx!(Icon { icon: FaBitcoinSign }),
            },

            IconName {
                name: "FaBlenderPhone".to_string(),
                icon: rsx!(Icon { icon: FaBlenderPhone }),
            },

            IconName {
                name: "FaBlender".to_string(),
                icon: rsx!(Icon { icon: FaBlender }),
            },

            IconName {
                name: "FaBlog".to_string(),
                icon: rsx!(Icon { icon: FaBlog }),
            },

            IconName {
                name: "FaBold".to_string(),
                icon: rsx!(Icon { icon: FaBold }),
            },

            IconName {
                name: "FaBoltLightning".to_string(),
                icon: rsx!(Icon { icon: FaBoltLightning }),
            },

            IconName {
                name: "FaBolt".to_string(),
                icon: rsx!(Icon { icon: FaBolt }),
            },

            IconName {
                name: "FaBomb".to_string(),
                icon: rsx!(Icon { icon: FaBomb }),
            },

            IconName {
                name: "FaBone".to_string(),
                icon: rsx!(Icon { icon: FaBone }),
            },

            IconName {
                name: "FaBong".to_string(),
                icon: rsx!(Icon { icon: FaBong }),
            },

            IconName {
                name: "FaBookAtlas".to_string(),
                icon: rsx!(Icon { icon: FaBookAtlas }),
            },

            IconName {
                name: "FaBookBible".to_string(),
                icon: rsx!(Icon { icon: FaBookBible }),
            },

            IconName {
                name: "FaBookBookmark".to_string(),
                icon: rsx!(Icon { icon: FaBookBookmark }),
            },

            IconName {
                name: "FaBookJournalWhills".to_string(),
                icon: rsx!(Icon { icon: FaBookJournalWhills }),
            },

            IconName {
                name: "FaBookMedical".to_string(),
                icon: rsx!(Icon { icon: FaBookMedical }),
            },

            IconName {
                name: "FaBookOpenReader".to_string(),
                icon: rsx!(Icon { icon: FaBookOpenReader }),
            },

            IconName {
                name: "FaBookOpen".to_string(),
                icon: rsx!(Icon { icon: FaBookOpen }),
            },

            IconName {
                name: "FaBookQuran".to_string(),
                icon: rsx!(Icon { icon: FaBookQuran }),
            },

            IconName {
                name: "FaBookSkull".to_string(),
                icon: rsx!(Icon { icon: FaBookSkull }),
            },

            IconName {
                name: "FaBook".to_string(),
                icon: rsx!(Icon { icon: FaBook }),
            },

            IconName {
                name: "FaBookmark".to_string(),
                icon: rsx!(Icon { icon: FaBookmark }),
            },

            IconName {
                name: "FaBorderAll".to_string(),
                icon: rsx!(Icon { icon: FaBorderAll }),
            },

            IconName {
                name: "FaBorderNone".to_string(),
                icon: rsx!(Icon { icon: FaBorderNone }),
            },

            IconName {
                name: "FaBorderTopLeft".to_string(),
                icon: rsx!(Icon { icon: FaBorderTopLeft }),
            },

            IconName {
                name: "FaBoreHole".to_string(),
                icon: rsx!(Icon { icon: FaBoreHole }),
            },

            IconName {
                name: "FaBottleDroplet".to_string(),
                icon: rsx!(Icon { icon: FaBottleDroplet }),
            },

            IconName {
                name: "FaBottleWater".to_string(),
                icon: rsx!(Icon { icon: FaBottleWater }),
            },

            IconName {
                name: "FaBowlFood".to_string(),
                icon: rsx!(Icon { icon: FaBowlFood }),
            },

            IconName {
                name: "FaBowlRice".to_string(),
                icon: rsx!(Icon { icon: FaBowlRice }),
            },

            IconName {
                name: "FaBowlingBall".to_string(),
                icon: rsx!(Icon { icon: FaBowlingBall }),
            },

            IconName {
                name: "FaBoxArchive".to_string(),
                icon: rsx!(Icon { icon: FaBoxArchive }),
            },

            IconName {
                name: "FaBoxOpen".to_string(),
                icon: rsx!(Icon { icon: FaBoxOpen }),
            },

            IconName {
                name: "FaBoxTissue".to_string(),
                icon: rsx!(Icon { icon: FaBoxTissue }),
            },

            IconName {
                name: "FaBox".to_string(),
                icon: rsx!(Icon { icon: FaBox }),
            },

            IconName {
                name: "FaBoxesPacking".to_string(),
                icon: rsx!(Icon { icon: FaBoxesPacking }),
            },

            IconName {
                name: "FaBoxesStacked".to_string(),
                icon: rsx!(Icon { icon: FaBoxesStacked }),
            },

            IconName {
                name: "FaBraille".to_string(),
                icon: rsx!(Icon { icon: FaBraille }),
            },

            IconName {
                name: "FaBrain".to_string(),
                icon: rsx!(Icon { icon: FaBrain }),
            },

            IconName {
                name: "FaBrazilianRealSign".to_string(),
                icon: rsx!(Icon { icon: FaBrazilianRealSign }),
            },

            IconName {
                name: "FaBreadSlice".to_string(),
                icon: rsx!(Icon { icon: FaBreadSlice }),
            },

            IconName {
                name: "FaBridgeCircleCheck".to_string(),
                icon: rsx!(Icon { icon: FaBridgeCircleCheck }),
            },

            IconName {
                name: "FaBridgeCircleExclamation".to_string(),
                icon: rsx!(Icon { icon: FaBridgeCircleExclamation }),
            },

            IconName {
                name: "FaBridgeCircleXmark".to_string(),
                icon: rsx!(Icon { icon: FaBridgeCircleXmark }),
            },

            IconName {
                name: "FaBridgeLock".to_string(),
                icon: rsx!(Icon { icon: FaBridgeLock }),
            },

            IconName {
                name: "FaBridgeWater".to_string(),
                icon: rsx!(Icon { icon: FaBridgeWater }),
            },

            IconName {
                name: "FaBridge".to_string(),
                icon: rsx!(Icon { icon: FaBridge }),
            },

            IconName {
                name: "FaBriefcaseMedical".to_string(),
                icon: rsx!(Icon { icon: FaBriefcaseMedical }),
            },

            IconName {
                name: "FaBriefcase".to_string(),
                icon: rsx!(Icon { icon: FaBriefcase }),
            },

            IconName {
                name: "FaBroomBall".to_string(),
                icon: rsx!(Icon { icon: FaBroomBall }),
            },

            IconName {
                name: "FaBroom".to_string(),
                icon: rsx!(Icon { icon: FaBroom }),
            },

            IconName {
                name: "FaBrush".to_string(),
                icon: rsx!(Icon { icon: FaBrush }),
            },

            IconName {
                name: "FaBucket".to_string(),
                icon: rsx!(Icon { icon: FaBucket }),
            },

            IconName {
                name: "FaBugSlash".to_string(),
                icon: rsx!(Icon { icon: FaBugSlash }),
            },

            IconName {
                name: "FaBug".to_string(),
                icon: rsx!(Icon { icon: FaBug }),
            },

            IconName {
                name: "FaBugs".to_string(),
                icon: rsx!(Icon { icon: FaBugs }),
            },

            IconName {
                name: "FaBuildingCircleArrowRight".to_string(),
                icon: rsx!(Icon { icon: FaBuildingCircleArrowRight }),
            },

            IconName {
                name: "FaBuildingCircleCheck".to_string(),
                icon: rsx!(Icon { icon: FaBuildingCircleCheck }),
            },

            IconName {
                name: "FaBuildingCircleExclamation".to_string(),
                icon: rsx!(Icon { icon: FaBuildingCircleExclamation }),
            },

            IconName {
                name: "FaBuildingCircleXmark".to_string(),
                icon: rsx!(Icon { icon: FaBuildingCircleXmark }),
            },

            IconName {
                name: "FaBuildingColumns".to_string(),
                icon: rsx!(Icon { icon: FaBuildingColumns }),
            },

            IconName {
                name: "FaBuildingFlag".to_string(),
                icon: rsx!(Icon { icon: FaBuildingFlag }),
            },

            IconName {
                name: "FaBuildingLock".to_string(),
                icon: rsx!(Icon { icon: FaBuildingLock }),
            },

            IconName {
                name: "FaBuildingNgo".to_string(),
                icon: rsx!(Icon { icon: FaBuildingNgo }),
            },

            IconName {
                name: "FaBuildingShield".to_string(),
                icon: rsx!(Icon { icon: FaBuildingShield }),
            },

            IconName {
                name: "FaBuildingUn".to_string(),
                icon: rsx!(Icon { icon: FaBuildingUn }),
            },

            IconName {
                name: "FaBuildingUser".to_string(),
                icon: rsx!(Icon { icon: FaBuildingUser }),
            },

            IconName {
                name: "FaBuildingWheat".to_string(),
                icon: rsx!(Icon { icon: FaBuildingWheat }),
            },

            IconName {
                name: "FaBuilding".to_string(),
                icon: rsx!(Icon { icon: FaBuilding }),
            },

            IconName {
                name: "FaBullhorn".to_string(),
                icon: rsx!(Icon { icon: FaBullhorn }),
            },

            IconName {
                name: "FaBullseye".to_string(),
                icon: rsx!(Icon { icon: FaBullseye }),
            },

            IconName {
                name: "FaBurger".to_string(),
                icon: rsx!(Icon { icon: FaBurger }),
            },

            IconName {
                name: "FaBurst".to_string(),
                icon: rsx!(Icon { icon: FaBurst }),
            },

            IconName {
                name: "FaBusSimple".to_string(),
                icon: rsx!(Icon { icon: FaBusSimple }),
            },

            IconName {
                name: "FaBus".to_string(),
                icon: rsx!(Icon { icon: FaBus }),
            },

            IconName {
                name: "FaBusinessTime".to_string(),
                icon: rsx!(Icon { icon: FaBusinessTime }),
            },

            IconName {
                name: "FaC".to_string(),
                icon: rsx!(Icon { icon: FaC }),
            },

            IconName {
                name: "FaCakeCandles".to_string(),
                icon: rsx!(Icon { icon: FaCakeCandles }),
            },

            IconName {
                name: "FaCalculator".to_string(),
                icon: rsx!(Icon { icon: FaCalculator }),
            },

            IconName {
                name: "FaCalendarCheck".to_string(),
                icon: rsx!(Icon { icon: FaCalendarCheck }),
            },

            IconName {
                name: "FaCalendarDay".to_string(),
                icon: rsx!(Icon { icon: FaCalendarDay }),
            },

            IconName {
                name: "FaCalendarDays".to_string(),
                icon: rsx!(Icon { icon: FaCalendarDays }),
            },

            IconName {
                name: "FaCalendarMinus".to_string(),
                icon: rsx!(Icon { icon: FaCalendarMinus }),
            },

            IconName {
                name: "FaCalendarPlus".to_string(),
                icon: rsx!(Icon { icon: FaCalendarPlus }),
            },

            IconName {
                name: "FaCalendarWeek".to_string(),
                icon: rsx!(Icon { icon: FaCalendarWeek }),
            },

            IconName {
                name: "FaCalendarXmark".to_string(),
                icon: rsx!(Icon { icon: FaCalendarXmark }),
            },

            IconName {
                name: "FaCalendar".to_string(),
                icon: rsx!(Icon { icon: FaCalendar }),
            },

            IconName {
                name: "FaCameraRetro".to_string(),
                icon: rsx!(Icon { icon: FaCameraRetro }),
            },

            IconName {
                name: "FaCameraRotate".to_string(),
                icon: rsx!(Icon { icon: FaCameraRotate }),
            },

            IconName {
                name: "FaCamera".to_string(),
                icon: rsx!(Icon { icon: FaCamera }),
            },

            IconName {
                name: "FaCampground".to_string(),
                icon: rsx!(Icon { icon: FaCampground }),
            },

            IconName {
                name: "FaCandyCane".to_string(),
                icon: rsx!(Icon { icon: FaCandyCane }),
            },

            IconName {
                name: "FaCannabis".to_string(),
                icon: rsx!(Icon { icon: FaCannabis }),
            },

            IconName {
                name: "FaCapsules".to_string(),
                icon: rsx!(Icon { icon: FaCapsules }),
            },

            IconName {
                name: "FaCarBattery".to_string(),
                icon: rsx!(Icon { icon: FaCarBattery }),
            },

            IconName {
                name: "FaCarBurst".to_string(),
                icon: rsx!(Icon { icon: FaCarBurst }),
            },

            IconName {
                name: "FaCarCrash".to_string(),
                icon: rsx!(Icon { icon: FaCarCrash }),
            },

            IconName {
                name: "FaCarOn".to_string(),
                icon: rsx!(Icon { icon: FaCarOn }),
            },

            IconName {
                name: "FaCarRear".to_string(),
                icon: rsx!(Icon { icon: FaCarRear }),
            },

            IconName {
                name: "FaCarSide".to_string(),
                icon: rsx!(Icon { icon: FaCarSide }),
            },

            IconName {
                name: "FaCarTunnel".to_string(),
                icon: rsx!(Icon { icon: FaCarTunnel }),
            },

            IconName {
                name: "FaCar".to_string(),
                icon: rsx!(Icon { icon: FaCar }),
            },

            IconName {
                name: "FaCaravan".to_string(),
                icon: rsx!(Icon { icon: FaCaravan }),
            },

            IconName {
                name: "FaCaretDown".to_string(),
                icon: rsx!(Icon { icon: FaCaretDown }),
            },

            IconName {
                name: "FaCaretLeft".to_string(),
                icon: rsx!(Icon { icon: FaCaretLeft }),
            },

            IconName {
                name: "FaCaretRight".to_string(),
                icon: rsx!(Icon { icon: FaCaretRight }),
            },

            IconName {
                name: "FaCaretUp".to_string(),
                icon: rsx!(Icon { icon: FaCaretUp }),
            },

            IconName {
                name: "FaCarrot".to_string(),
                icon: rsx!(Icon { icon: FaCarrot }),
            },

            IconName {
                name: "FaCartArrowDown".to_string(),
                icon: rsx!(Icon { icon: FaCartArrowDown }),
            },

            IconName {
                name: "FaCartFlatbedSuitcase".to_string(),
                icon: rsx!(Icon { icon: FaCartFlatbedSuitcase }),
            },

            IconName {
                name: "FaCartFlatbed".to_string(),
                icon: rsx!(Icon { icon: FaCartFlatbed }),
            },

            IconName {
                name: "FaCartPlus".to_string(),
                icon: rsx!(Icon { icon: FaCartPlus }),
            },

            IconName {
                name: "FaCartShopping".to_string(),
                icon: rsx!(Icon { icon: FaCartShopping }),
            },

            IconName {
                name: "FaCashRegister".to_string(),
                icon: rsx!(Icon { icon: FaCashRegister }),
            },

            IconName {
                name: "FaCat".to_string(),
                icon: rsx!(Icon { icon: FaCat }),
            },

            IconName {
                name: "FaCediSign".to_string(),
                icon: rsx!(Icon { icon: FaCediSign }),
            },

            IconName {
                name: "FaCentSign".to_string(),
                icon: rsx!(Icon { icon: FaCentSign }),
            },

            IconName {
                name: "FaCertificate".to_string(),
                icon: rsx!(Icon { icon: FaCertificate }),
            },

            IconName {
                name: "FaChair".to_string(),
                icon: rsx!(Icon { icon: FaChair }),
            },

            IconName {
                name: "FaChalkboardUser".to_string(),
                icon: rsx!(Icon { icon: FaChalkboardUser }),
            },

            IconName {
                name: "FaChalkboard".to_string(),
                icon: rsx!(Icon { icon: FaChalkboard }),
            },

            IconName {
                name: "FaChampagneGlasses".to_string(),
                icon: rsx!(Icon { icon: FaChampagneGlasses }),
            },

            IconName {
                name: "FaChargingStation".to_string(),
                icon: rsx!(Icon { icon: FaChargingStation }),
            },

            IconName {
                name: "FaChartArea".to_string(),
                icon: rsx!(Icon { icon: FaChartArea }),
            },

            IconName {
                name: "FaChartBar".to_string(),
                icon: rsx!(Icon { icon: FaChartBar }),
            },

            IconName {
                name: "FaChartColumn".to_string(),
                icon: rsx!(Icon { icon: FaChartColumn }),
            },

            IconName {
                name: "FaChartGantt".to_string(),
                icon: rsx!(Icon { icon: FaChartGantt }),
            },

            IconName {
                name: "FaChartLine".to_string(),
                icon: rsx!(Icon { icon: FaChartLine }),
            },

            IconName {
                name: "FaChartPie".to_string(),
                icon: rsx!(Icon { icon: FaChartPie }),
            },

            IconName {
                name: "FaChartSimple".to_string(),
                icon: rsx!(Icon { icon: FaChartSimple }),
            },

            IconName {
                name: "FaCheckDouble".to_string(),
                icon: rsx!(Icon { icon: FaCheckDouble }),
            },

            IconName {
                name: "FaCheckToSlot".to_string(),
                icon: rsx!(Icon { icon: FaCheckToSlot }),
            },

            IconName {
                name: "FaCheck".to_string(),
                icon: rsx!(Icon { icon: FaCheck }),
            },

            IconName {
                name: "FaCheese".to_string(),
                icon: rsx!(Icon { icon: FaCheese }),
            },

            IconName {
                name: "FaChessBishop".to_string(),
                icon: rsx!(Icon { icon: FaChessBishop }),
            },

            IconName {
                name: "FaChessBoard".to_string(),
                icon: rsx!(Icon { icon: FaChessBoard }),
            },

            IconName {
                name: "FaChessKing".to_string(),
                icon: rsx!(Icon { icon: FaChessKing }),
            },

            IconName {
                name: "FaChessKnight".to_string(),
                icon: rsx!(Icon { icon: FaChessKnight }),
            },

            IconName {
                name: "FaChessPawn".to_string(),
                icon: rsx!(Icon { icon: FaChessPawn }),
            },

            IconName {
                name: "FaChessQueen".to_string(),
                icon: rsx!(Icon { icon: FaChessQueen }),
            },

            IconName {
                name: "FaChessRook".to_string(),
                icon: rsx!(Icon { icon: FaChessRook }),
            },

            IconName {
                name: "FaChess".to_string(),
                icon: rsx!(Icon { icon: FaChess }),
            },

            IconName {
                name: "FaChevronDown".to_string(),
                icon: rsx!(Icon { icon: FaChevronDown }),
            },

            IconName {
                name: "FaChevronLeft".to_string(),
                icon: rsx!(Icon { icon: FaChevronLeft }),
            },

            IconName {
                name: "FaChevronRight".to_string(),
                icon: rsx!(Icon { icon: FaChevronRight }),
            },

            IconName {
                name: "FaChevronUp".to_string(),
                icon: rsx!(Icon { icon: FaChevronUp }),
            },

            IconName {
                name: "FaChildDress".to_string(),
                icon: rsx!(Icon { icon: FaChildDress }),
            },

            IconName {
                name: "FaChildReaching".to_string(),
                icon: rsx!(Icon { icon: FaChildReaching }),
            },

            IconName {
                name: "FaChildRifle".to_string(),
                icon: rsx!(Icon { icon: FaChildRifle }),
            },

            IconName {
                name: "FaChild".to_string(),
                icon: rsx!(Icon { icon: FaChild }),
            },

            IconName {
                name: "FaChildren".to_string(),
                icon: rsx!(Icon { icon: FaChildren }),
            },

            IconName {
                name: "FaChurch".to_string(),
                icon: rsx!(Icon { icon: FaChurch }),
            },

            IconName {
                name: "FaCircleArrowDown".to_string(),
                icon: rsx!(Icon { icon: FaCircleArrowDown }),
            },

            IconName {
                name: "FaCircleArrowLeft".to_string(),
                icon: rsx!(Icon { icon: FaCircleArrowLeft }),
            },

            IconName {
                name: "FaCircleArrowRight".to_string(),
                icon: rsx!(Icon { icon: FaCircleArrowRight }),
            },

            IconName {
                name: "FaCircleArrowUp".to_string(),
                icon: rsx!(Icon { icon: FaCircleArrowUp }),
            },

            IconName {
                name: "FaCircleCheck".to_string(),
                icon: rsx!(Icon { icon: FaCircleCheck }),
            },

            IconName {
                name: "FaCircleChevronDown".to_string(),
                icon: rsx!(Icon { icon: FaCircleChevronDown }),
            },

            IconName {
                name: "FaCircleChevronLeft".to_string(),
                icon: rsx!(Icon { icon: FaCircleChevronLeft }),
            },

            IconName {
                name: "FaCircleChevronRight".to_string(),
                icon: rsx!(Icon { icon: FaCircleChevronRight }),
            },

            IconName {
                name: "FaCircleChevronUp".to_string(),
                icon: rsx!(Icon { icon: FaCircleChevronUp }),
            },

            IconName {
                name: "FaCircleDollarToSlot".to_string(),
                icon: rsx!(Icon { icon: FaCircleDollarToSlot }),
            },

            IconName {
                name: "FaCircleDot".to_string(),
                icon: rsx!(Icon { icon: FaCircleDot }),
            },

            IconName {
                name: "FaCircleDown".to_string(),
                icon: rsx!(Icon { icon: FaCircleDown }),
            },

            IconName {
                name: "FaCircleExclamation".to_string(),
                icon: rsx!(Icon { icon: FaCircleExclamation }),
            },

            IconName {
                name: "FaCircleH".to_string(),
                icon: rsx!(Icon { icon: FaCircleH }),
            },

            IconName {
                name: "FaCircleHalfStroke".to_string(),
                icon: rsx!(Icon { icon: FaCircleHalfStroke }),
            },

            IconName {
                name: "FaCircleInfo".to_string(),
                icon: rsx!(Icon { icon: FaCircleInfo }),
            },

            IconName {
                name: "FaCircleLeft".to_string(),
                icon: rsx!(Icon { icon: FaCircleLeft }),
            },

            IconName {
                name: "FaCircleMinus".to_string(),
                icon: rsx!(Icon { icon: FaCircleMinus }),
            },

            IconName {
                name: "FaCircleNodes".to_string(),
                icon: rsx!(Icon { icon: FaCircleNodes }),
            },

            IconName {
                name: "FaCircleNotch".to_string(),
                icon: rsx!(Icon { icon: FaCircleNotch }),
            },

            IconName {
                name: "FaCirclePause".to_string(),
                icon: rsx!(Icon { icon: FaCirclePause }),
            },

            IconName {
                name: "FaCirclePlay".to_string(),
                icon: rsx!(Icon { icon: FaCirclePlay }),
            },

            IconName {
                name: "FaCirclePlus".to_string(),
                icon: rsx!(Icon { icon: FaCirclePlus }),
            },

            IconName {
                name: "FaCircleQuestion".to_string(),
                icon: rsx!(Icon { icon: FaCircleQuestion }),
            },

            IconName {
                name: "FaCircleRadiation".to_string(),
                icon: rsx!(Icon { icon: FaCircleRadiation }),
            },

            IconName {
                name: "FaCircleRight".to_string(),
                icon: rsx!(Icon { icon: FaCircleRight }),
            },

            IconName {
                name: "FaCircleStop".to_string(),
                icon: rsx!(Icon { icon: FaCircleStop }),
            },

            IconName {
                name: "FaCircleUp".to_string(),
                icon: rsx!(Icon { icon: FaCircleUp }),
            },

            IconName {
                name: "FaCircleUser".to_string(),
                icon: rsx!(Icon { icon: FaCircleUser }),
            },

            IconName {
                name: "FaCircleXmark".to_string(),
                icon: rsx!(Icon { icon: FaCircleXmark }),
            },

            IconName {
                name: "FaCircle".to_string(),
                icon: rsx!(Icon { icon: FaCircle }),
            },

            IconName {
                name: "FaCity".to_string(),
                icon: rsx!(Icon { icon: FaCity }),
            },

            IconName {
                name: "FaClapperboard".to_string(),
                icon: rsx!(Icon { icon: FaClapperboard }),
            },

            IconName {
                name: "FaClipboardCheck".to_string(),
                icon: rsx!(Icon { icon: FaClipboardCheck }),
            },

            IconName {
                name: "FaClipboardList".to_string(),
                icon: rsx!(Icon { icon: FaClipboardList }),
            },

            IconName {
                name: "FaClipboardQuestion".to_string(),
                icon: rsx!(Icon { icon: FaClipboardQuestion }),
            },

            IconName {
                name: "FaClipboardUser".to_string(),
                icon: rsx!(Icon { icon: FaClipboardUser }),
            },

            IconName {
                name: "FaClipboard".to_string(),
                icon: rsx!(Icon { icon: FaClipboard }),
            },

            IconName {
                name: "FaClockRotateLeft".to_string(),
                icon: rsx!(Icon { icon: FaClockRotateLeft }),
            },

            IconName {
                name: "FaClock".to_string(),
                icon: rsx!(Icon { icon: FaClock }),
            },

            IconName {
                name: "FaClone".to_string(),
                icon: rsx!(Icon { icon: FaClone }),
            },

            IconName {
                name: "FaClosedCaptioning".to_string(),
                icon: rsx!(Icon { icon: FaClosedCaptioning }),
            },

            IconName {
                name: "FaCloudArrowDown".to_string(),
                icon: rsx!(Icon { icon: FaCloudArrowDown }),
            },

            IconName {
                name: "FaCloudArrowUp".to_string(),
                icon: rsx!(Icon { icon: FaCloudArrowUp }),
            },

            IconName {
                name: "FaCloudBolt".to_string(),
                icon: rsx!(Icon { icon: FaCloudBolt }),
            },

            IconName {
                name: "FaCloudMeatball".to_string(),
                icon: rsx!(Icon { icon: FaCloudMeatball }),
            },

            IconName {
                name: "FaCloudMoonRain".to_string(),
                icon: rsx!(Icon { icon: FaCloudMoonRain }),
            },

            IconName {
                name: "FaCloudMoon".to_string(),
                icon: rsx!(Icon { icon: FaCloudMoon }),
            },

            IconName {
                name: "FaCloudRain".to_string(),
                icon: rsx!(Icon { icon: FaCloudRain }),
            },

            IconName {
                name: "FaCloudShowersHeavy".to_string(),
                icon: rsx!(Icon { icon: FaCloudShowersHeavy }),
            },

            IconName {
                name: "FaCloudShowersWater".to_string(),
                icon: rsx!(Icon { icon: FaCloudShowersWater }),
            },

            IconName {
                name: "FaCloudSunRain".to_string(),
                icon: rsx!(Icon { icon: FaCloudSunRain }),
            },

            IconName {
                name: "FaCloudSun".to_string(),
                icon: rsx!(Icon { icon: FaCloudSun }),
            },

            IconName {
                name: "FaCloud".to_string(),
                icon: rsx!(Icon { icon: FaCloud }),
            },

            IconName {
                name: "FaClover".to_string(),
                icon: rsx!(Icon { icon: FaClover }),
            },

            IconName {
                name: "FaCodeBranch".to_string(),
                icon: rsx!(Icon { icon: FaCodeBranch }),
            },

            IconName {
                name: "FaCodeCommit".to_string(),
                icon: rsx!(Icon { icon: FaCodeCommit }),
            },

            IconName {
                name: "FaCodeCompare".to_string(),
                icon: rsx!(Icon { icon: FaCodeCompare }),
            },

            IconName {
                name: "FaCodeFork".to_string(),
                icon: rsx!(Icon { icon: FaCodeFork }),
            },

            IconName {
                name: "FaCodeMerge".to_string(),
                icon: rsx!(Icon { icon: FaCodeMerge }),
            },

            IconName {
                name: "FaCodePullRequest".to_string(),
                icon: rsx!(Icon { icon: FaCodePullRequest }),
            },

            IconName {
                name: "FaCode".to_string(),
                icon: rsx!(Icon { icon: FaCode }),
            },

            IconName {
                name: "FaCoins".to_string(),
                icon: rsx!(Icon { icon: FaCoins }),
            },

            IconName {
                name: "FaColonSign".to_string(),
                icon: rsx!(Icon { icon: FaColonSign }),
            },

            IconName {
                name: "FaCommentDollar".to_string(),
                icon: rsx!(Icon { icon: FaCommentDollar }),
            },

            IconName {
                name: "FaCommentDots".to_string(),
                icon: rsx!(Icon { icon: FaCommentDots }),
            },

            IconName {
                name: "FaCommentMedical".to_string(),
                icon: rsx!(Icon { icon: FaCommentMedical }),
            },

            IconName {
                name: "FaCommentSlash".to_string(),
                icon: rsx!(Icon { icon: FaCommentSlash }),
            },

            IconName {
                name: "FaCommentSms".to_string(),
                icon: rsx!(Icon { icon: FaCommentSms }),
            },

            IconName {
                name: "FaComment".to_string(),
                icon: rsx!(Icon { icon: FaComment }),
            },

            IconName {
                name: "FaCommentsDollar".to_string(),
                icon: rsx!(Icon { icon: FaCommentsDollar }),
            },

            IconName {
                name: "FaComments".to_string(),
                icon: rsx!(Icon { icon: FaComments }),
            },

            IconName {
                name: "FaCompactDisc".to_string(),
                icon: rsx!(Icon { icon: FaCompactDisc }),
            },

            IconName {
                name: "FaCompassDrafting".to_string(),
                icon: rsx!(Icon { icon: FaCompassDrafting }),
            },

            IconName {
                name: "FaCompass".to_string(),
                icon: rsx!(Icon { icon: FaCompass }),
            },

            IconName {
                name: "FaCompress".to_string(),
                icon: rsx!(Icon { icon: FaCompress }),
            },

            IconName {
                name: "FaComputerMouse".to_string(),
                icon: rsx!(Icon { icon: FaComputerMouse }),
            },

            IconName {
                name: "FaComputer".to_string(),
                icon: rsx!(Icon { icon: FaComputer }),
            },

            IconName {
                name: "FaCookieBite".to_string(),
                icon: rsx!(Icon { icon: FaCookieBite }),
            },

            IconName {
                name: "FaCookie".to_string(),
                icon: rsx!(Icon { icon: FaCookie }),
            },

            IconName {
                name: "FaCopy".to_string(),
                icon: rsx!(Icon { icon: FaCopy }),
            },

            IconName {
                name: "FaCopyright".to_string(),
                icon: rsx!(Icon { icon: FaCopyright }),
            },

            IconName {
                name: "FaCouch".to_string(),
                icon: rsx!(Icon { icon: FaCouch }),
            },

            IconName {
                name: "FaCow".to_string(),
                icon: rsx!(Icon { icon: FaCow }),
            },

            IconName {
                name: "FaCreditCard".to_string(),
                icon: rsx!(Icon { icon: FaCreditCard }),
            },

            IconName {
                name: "FaCropSimple".to_string(),
                icon: rsx!(Icon { icon: FaCropSimple }),
            },

            IconName {
                name: "FaCrop".to_string(),
                icon: rsx!(Icon { icon: FaCrop }),
            },

            IconName {
                name: "FaCross".to_string(),
                icon: rsx!(Icon { icon: FaCross }),
            },

            IconName {
                name: "FaCrosshairs".to_string(),
                icon: rsx!(Icon { icon: FaCrosshairs }),
            },

            IconName {
                name: "FaCrow".to_string(),
                icon: rsx!(Icon { icon: FaCrow }),
            },

            IconName {
                name: "FaCrown".to_string(),
                icon: rsx!(Icon { icon: FaCrown }),
            },

            IconName {
                name: "FaCrutch".to_string(),
                icon: rsx!(Icon { icon: FaCrutch }),
            },

            IconName {
                name: "FaCruzeiroSign".to_string(),
                icon: rsx!(Icon { icon: FaCruzeiroSign }),
            },

            IconName {
                name: "FaCube".to_string(),
                icon: rsx!(Icon { icon: FaCube }),
            },

            IconName {
                name: "FaCubesStacked".to_string(),
                icon: rsx!(Icon { icon: FaCubesStacked }),
            },

            IconName {
                name: "FaCubes".to_string(),
                icon: rsx!(Icon { icon: FaCubes }),
            },

            IconName {
                name: "FaD".to_string(),
                icon: rsx!(Icon { icon: FaD }),
            },

            IconName {
                name: "FaDatabase".to_string(),
                icon: rsx!(Icon { icon: FaDatabase }),
            },

            IconName {
                name: "FaDeleteLeft".to_string(),
                icon: rsx!(Icon { icon: FaDeleteLeft }),
            },

            IconName {
                name: "FaDemocrat".to_string(),
                icon: rsx!(Icon { icon: FaDemocrat }),
            },

            IconName {
                name: "FaDesktop".to_string(),
                icon: rsx!(Icon { icon: FaDesktop }),
            },

            IconName {
                name: "FaDharmachakra".to_string(),
                icon: rsx!(Icon { icon: FaDharmachakra }),
            },

            IconName {
                name: "FaDiagramNext".to_string(),
                icon: rsx!(Icon { icon: FaDiagramNext }),
            },

            IconName {
                name: "FaDiagramPredecessor".to_string(),
                icon: rsx!(Icon { icon: FaDiagramPredecessor }),
            },

            IconName {
                name: "FaDiagramProject".to_string(),
                icon: rsx!(Icon { icon: FaDiagramProject }),
            },

            IconName {
                name: "FaDiagramSuccessor".to_string(),
                icon: rsx!(Icon { icon: FaDiagramSuccessor }),
            },

            IconName {
                name: "FaDiamondTurnRight".to_string(),
                icon: rsx!(Icon { icon: FaDiamondTurnRight }),
            },

            IconName {
                name: "FaDiamond".to_string(),
                icon: rsx!(Icon { icon: FaDiamond }),
            },

            IconName {
                name: "FaDiceD20".to_string(),
                icon: rsx!(Icon { icon: FaDiceD20 }),
            },

            IconName {
                name: "FaDiceD6".to_string(),
                icon: rsx!(Icon { icon: FaDiceD6 }),
            },

            IconName {
                name: "FaDiceFive".to_string(),
                icon: rsx!(Icon { icon: FaDiceFive }),
            },

            IconName {
                name: "FaDiceFour".to_string(),
                icon: rsx!(Icon { icon: FaDiceFour }),
            },

            IconName {
                name: "FaDiceOne".to_string(),
                icon: rsx!(Icon { icon: FaDiceOne }),
            },

            IconName {
                name: "FaDiceSix".to_string(),
                icon: rsx!(Icon { icon: FaDiceSix }),
            },

            IconName {
                name: "FaDiceThree".to_string(),
                icon: rsx!(Icon { icon: FaDiceThree }),
            },

            IconName {
                name: "FaDiceTwo".to_string(),
                icon: rsx!(Icon { icon: FaDiceTwo }),
            },

            IconName {
                name: "FaDice".to_string(),
                icon: rsx!(Icon { icon: FaDice }),
            },

            IconName {
                name: "FaDisease".to_string(),
                icon: rsx!(Icon { icon: FaDisease }),
            },

            IconName {
                name: "FaDisplay".to_string(),
                icon: rsx!(Icon { icon: FaDisplay }),
            },

            IconName {
                name: "FaDivide".to_string(),
                icon: rsx!(Icon { icon: FaDivide }),
            },

            IconName {
                name: "FaDna".to_string(),
                icon: rsx!(Icon { icon: FaDna }),
            },

            IconName {
                name: "FaDog".to_string(),
                icon: rsx!(Icon { icon: FaDog }),
            },

            IconName {
                name: "FaDollarSign".to_string(),
                icon: rsx!(Icon { icon: FaDollarSign }),
            },

            IconName {
                name: "FaDolly".to_string(),
                icon: rsx!(Icon { icon: FaDolly }),
            },

            IconName {
                name: "FaDongSign".to_string(),
                icon: rsx!(Icon { icon: FaDongSign }),
            },

            IconName {
                name: "FaDoorClosed".to_string(),
                icon: rsx!(Icon { icon: FaDoorClosed }),
            },

            IconName {
                name: "FaDoorOpen".to_string(),
                icon: rsx!(Icon { icon: FaDoorOpen }),
            },

            IconName {
                name: "FaDove".to_string(),
                icon: rsx!(Icon { icon: FaDove }),
            },

            IconName {
                name: "FaDownLeftAndUpRightToCenter".to_string(),
                icon: rsx!(Icon { icon: FaDownLeftAndUpRightToCenter }),
            },

            IconName {
                name: "FaDownLong".to_string(),
                icon: rsx!(Icon { icon: FaDownLong }),
            },

            IconName {
                name: "FaDownload".to_string(),
                icon: rsx!(Icon { icon: FaDownload }),
            },

            IconName {
                name: "FaDragon".to_string(),
                icon: rsx!(Icon { icon: FaDragon }),
            },

            IconName {
                name: "FaDrawPolygon".to_string(),
                icon: rsx!(Icon { icon: FaDrawPolygon }),
            },

            IconName {
                name: "FaDropletSlash".to_string(),
                icon: rsx!(Icon { icon: FaDropletSlash }),
            },

            IconName {
                name: "FaDroplet".to_string(),
                icon: rsx!(Icon { icon: FaDroplet }),
            },

            IconName {
                name: "FaDrumSteelpan".to_string(),
                icon: rsx!(Icon { icon: FaDrumSteelpan }),
            },

            IconName {
                name: "FaDrum".to_string(),
                icon: rsx!(Icon { icon: FaDrum }),
            },

            IconName {
                name: "FaDrumstickBite".to_string(),
                icon: rsx!(Icon { icon: FaDrumstickBite }),
            },

            IconName {
                name: "FaDumbbell".to_string(),
                icon: rsx!(Icon { icon: FaDumbbell }),
            },

            IconName {
                name: "FaDumpsterFire".to_string(),
                icon: rsx!(Icon { icon: FaDumpsterFire }),
            },

            IconName {
                name: "FaDumpster".to_string(),
                icon: rsx!(Icon { icon: FaDumpster }),
            },

            IconName {
                name: "FaDungeon".to_string(),
                icon: rsx!(Icon { icon: FaDungeon }),
            },

            IconName {
                name: "FaE".to_string(),
                icon: rsx!(Icon { icon: FaE }),
            },

            IconName {
                name: "FaEarDeaf".to_string(),
                icon: rsx!(Icon { icon: FaEarDeaf }),
            },

            IconName {
                name: "FaEarListen".to_string(),
                icon: rsx!(Icon { icon: FaEarListen }),
            },

            IconName {
                name: "FaEarthAfrica".to_string(),
                icon: rsx!(Icon { icon: FaEarthAfrica }),
            },

            IconName {
                name: "FaEarthAmericas".to_string(),
                icon: rsx!(Icon { icon: FaEarthAmericas }),
            },

            IconName {
                name: "FaEarthAsia".to_string(),
                icon: rsx!(Icon { icon: FaEarthAsia }),
            },

            IconName {
                name: "FaEarthEurope".to_string(),
                icon: rsx!(Icon { icon: FaEarthEurope }),
            },

            IconName {
                name: "FaEarthOceania".to_string(),
                icon: rsx!(Icon { icon: FaEarthOceania }),
            },

            IconName {
                name: "FaEgg".to_string(),
                icon: rsx!(Icon { icon: FaEgg }),
            },

            IconName {
                name: "FaEject".to_string(),
                icon: rsx!(Icon { icon: FaEject }),
            },

            IconName {
                name: "FaElevator".to_string(),
                icon: rsx!(Icon { icon: FaElevator }),
            },

            IconName {
                name: "FaEllipsisVertical".to_string(),
                icon: rsx!(Icon { icon: FaEllipsisVertical }),
            },

            IconName {
                name: "FaEllipsis".to_string(),
                icon: rsx!(Icon { icon: FaEllipsis }),
            },

            IconName {
                name: "FaEnvelopeCircleCheck".to_string(),
                icon: rsx!(Icon { icon: FaEnvelopeCircleCheck }),
            },

            IconName {
                name: "FaEnvelopeOpenText".to_string(),
                icon: rsx!(Icon { icon: FaEnvelopeOpenText }),
            },

            IconName {
                name: "FaEnvelopeOpen".to_string(),
                icon: rsx!(Icon { icon: FaEnvelopeOpen }),
            },

            IconName {
                name: "FaEnvelope".to_string(),
                icon: rsx!(Icon { icon: FaEnvelope }),
            },

            IconName {
                name: "FaEnvelopesBulk".to_string(),
                icon: rsx!(Icon { icon: FaEnvelopesBulk }),
            },

            IconName {
                name: "FaEquals".to_string(),
                icon: rsx!(Icon { icon: FaEquals }),
            },

            IconName {
                name: "FaEraser".to_string(),
                icon: rsx!(Icon { icon: FaEraser }),
            },

            IconName {
                name: "FaEthernet".to_string(),
                icon: rsx!(Icon { icon: FaEthernet }),
            },

            IconName {
                name: "FaEuroSign".to_string(),
                icon: rsx!(Icon { icon: FaEuroSign }),
            },

            IconName {
                name: "FaExclamation".to_string(),
                icon: rsx!(Icon { icon: FaExclamation }),
            },

            IconName {
                name: "FaExpand".to_string(),
                icon: rsx!(Icon { icon: FaExpand }),
            },

            IconName {
                name: "FaExplosion".to_string(),
                icon: rsx!(Icon { icon: FaExplosion }),
            },

            IconName {
                name: "FaEyeDropper".to_string(),
                icon: rsx!(Icon { icon: FaEyeDropper }),
            },

            IconName {
                name: "FaEyeLowVision".to_string(),
                icon: rsx!(Icon { icon: FaEyeLowVision }),
            },

            IconName {
                name: "FaEyeSlash".to_string(),
                icon: rsx!(Icon { icon: FaEyeSlash }),
            },

            IconName {
                name: "FaEye".to_string(),
                icon: rsx!(Icon { icon: FaEye }),
            },

            IconName {
                name: "FaF".to_string(),
                icon: rsx!(Icon { icon: FaF }),
            },

            IconName {
                name: "FaFaceAngry".to_string(),
                icon: rsx!(Icon { icon: FaFaceAngry }),
            },

            IconName {
                name: "FaFaceDizzy".to_string(),
                icon: rsx!(Icon { icon: FaFaceDizzy }),
            },

            IconName {
                name: "FaFaceFlushed".to_string(),
                icon: rsx!(Icon { icon: FaFaceFlushed }),
            },

            IconName {
                name: "FaFaceFrownOpen".to_string(),
                icon: rsx!(Icon { icon: FaFaceFrownOpen }),
            },

            IconName {
                name: "FaFaceFrown".to_string(),
                icon: rsx!(Icon { icon: FaFaceFrown }),
            },

            IconName {
                name: "FaFaceGrimace".to_string(),
                icon: rsx!(Icon { icon: FaFaceGrimace }),
            },

            IconName {
                name: "FaFaceGrinBeamSweat".to_string(),
                icon: rsx!(Icon { icon: FaFaceGrinBeamSweat }),
            },

            IconName {
                name: "FaFaceGrinBeam".to_string(),
                icon: rsx!(Icon { icon: FaFaceGrinBeam }),
            },

            IconName {
                name: "FaFaceGrinHearts".to_string(),
                icon: rsx!(Icon { icon: FaFaceGrinHearts }),
            },

            IconName {
                name: "FaFaceGrinSquintTears".to_string(),
                icon: rsx!(Icon { icon: FaFaceGrinSquintTears }),
            },

            IconName {
                name: "FaFaceGrinSquint".to_string(),
                icon: rsx!(Icon { icon: FaFaceGrinSquint }),
            },

            IconName {
                name: "FaFaceGrinStars".to_string(),
                icon: rsx!(Icon { icon: FaFaceGrinStars }),
            },

            IconName {
                name: "FaFaceGrinTears".to_string(),
                icon: rsx!(Icon { icon: FaFaceGrinTears }),
            },

            IconName {
                name: "FaFaceGrinTongueSquint".to_string(),
                icon: rsx!(Icon { icon: FaFaceGrinTongueSquint }),
            },

            IconName {
                name: "FaFaceGrinTongueWink".to_string(),
                icon: rsx!(Icon { icon: FaFaceGrinTongueWink }),
            },

            IconName {
                name: "FaFaceGrinTongue".to_string(),
                icon: rsx!(Icon { icon: FaFaceGrinTongue }),
            },

            IconName {
                name: "FaFaceGrinWide".to_string(),
                icon: rsx!(Icon { icon: FaFaceGrinWide }),
            },

            IconName {
                name: "FaFaceGrinWink".to_string(),
                icon: rsx!(Icon { icon: FaFaceGrinWink }),
            },

            IconName {
                name: "FaFaceGrin".to_string(),
                icon: rsx!(Icon { icon: FaFaceGrin }),
            },

            IconName {
                name: "FaFaceKissBeam".to_string(),
                icon: rsx!(Icon { icon: FaFaceKissBeam }),
            },

            IconName {
                name: "FaFaceKissWinkHeart".to_string(),
                icon: rsx!(Icon { icon: FaFaceKissWinkHeart }),
            },

            IconName {
                name: "FaFaceKiss".to_string(),
                icon: rsx!(Icon { icon: FaFaceKiss }),
            },

            IconName {
                name: "FaFaceLaughBeam".to_string(),
                icon: rsx!(Icon { icon: FaFaceLaughBeam }),
            },

            IconName {
                name: "FaFaceLaughSquint".to_string(),
                icon: rsx!(Icon { icon: FaFaceLaughSquint }),
            },

            IconName {
                name: "FaFaceLaughWink".to_string(),
                icon: rsx!(Icon { icon: FaFaceLaughWink }),
            },

            IconName {
                name: "FaFaceLaugh".to_string(),
                icon: rsx!(Icon { icon: FaFaceLaugh }),
            },

            IconName {
                name: "FaFaceMehBlank".to_string(),
                icon: rsx!(Icon { icon: FaFaceMehBlank }),
            },

            IconName {
                name: "FaFaceMeh".to_string(),
                icon: rsx!(Icon { icon: FaFaceMeh }),
            },

            IconName {
                name: "FaFaceRollingEyes".to_string(),
                icon: rsx!(Icon { icon: FaFaceRollingEyes }),
            },

            IconName {
                name: "FaFaceSadCry".to_string(),
                icon: rsx!(Icon { icon: FaFaceSadCry }),
            },

            IconName {
                name: "FaFaceSadTear".to_string(),
                icon: rsx!(Icon { icon: FaFaceSadTear }),
            },

            IconName {
                name: "FaFaceSmileBeam".to_string(),
                icon: rsx!(Icon { icon: FaFaceSmileBeam }),
            },

            IconName {
                name: "FaFaceSmileWink".to_string(),
                icon: rsx!(Icon { icon: FaFaceSmileWink }),
            },

            IconName {
                name: "FaFaceSmile".to_string(),
                icon: rsx!(Icon { icon: FaFaceSmile }),
            },

            IconName {
                name: "FaFaceSurprise".to_string(),
                icon: rsx!(Icon { icon: FaFaceSurprise }),
            },

            IconName {
                name: "FaFaceTired".to_string(),
                icon: rsx!(Icon { icon: FaFaceTired }),
            },

            IconName {
                name: "FaFan".to_string(),
                icon: rsx!(Icon { icon: FaFan }),
            },

            IconName {
                name: "FaFaucetDrip".to_string(),
                icon: rsx!(Icon { icon: FaFaucetDrip }),
            },

            IconName {
                name: "FaFaucet".to_string(),
                icon: rsx!(Icon { icon: FaFaucet }),
            },

            IconName {
                name: "FaFax".to_string(),
                icon: rsx!(Icon { icon: FaFax }),
            },

            IconName {
                name: "FaFeatherPointed".to_string(),
                icon: rsx!(Icon { icon: FaFeatherPointed }),
            },

            IconName {
                name: "FaFeather".to_string(),
                icon: rsx!(Icon { icon: FaFeather }),
            },

            IconName {
                name: "FaFerry".to_string(),
                icon: rsx!(Icon { icon: FaFerry }),
            },

            IconName {
                name: "FaFileArrowDown".to_string(),
                icon: rsx!(Icon { icon: FaFileArrowDown }),
            },

            IconName {
                name: "FaFileArrowUp".to_string(),
                icon: rsx!(Icon { icon: FaFileArrowUp }),
            },

            IconName {
                name: "FaFileAudio".to_string(),
                icon: rsx!(Icon { icon: FaFileAudio }),
            },

            IconName {
                name: "FaFileCircleCheck".to_string(),
                icon: rsx!(Icon { icon: FaFileCircleCheck }),
            },

            IconName {
                name: "FaFileCircleExclamation".to_string(),
                icon: rsx!(Icon { icon: FaFileCircleExclamation }),
            },

            IconName {
                name: "FaFileCircleMinus".to_string(),
                icon: rsx!(Icon { icon: FaFileCircleMinus }),
            },

            IconName {
                name: "FaFileCirclePlus".to_string(),
                icon: rsx!(Icon { icon: FaFileCirclePlus }),
            },

            IconName {
                name: "FaFileCircleQuestion".to_string(),
                icon: rsx!(Icon { icon: FaFileCircleQuestion }),
            },

            IconName {
                name: "FaFileCircleXmark".to_string(),
                icon: rsx!(Icon { icon: FaFileCircleXmark }),
            },

            IconName {
                name: "FaFileCode".to_string(),
                icon: rsx!(Icon { icon: FaFileCode }),
            },

            IconName {
                name: "FaFileContract".to_string(),
                icon: rsx!(Icon { icon: FaFileContract }),
            },

            IconName {
                name: "FaFileCsv".to_string(),
                icon: rsx!(Icon { icon: FaFileCsv }),
            },

            IconName {
                name: "FaFileExcel".to_string(),
                icon: rsx!(Icon { icon: FaFileExcel }),
            },

            IconName {
                name: "FaFileExport".to_string(),
                icon: rsx!(Icon { icon: FaFileExport }),
            },

            IconName {
                name: "FaFileImage".to_string(),
                icon: rsx!(Icon { icon: FaFileImage }),
            },

            IconName {
                name: "FaFileImport".to_string(),
                icon: rsx!(Icon { icon: FaFileImport }),
            },

            IconName {
                name: "FaFileInvoiceDollar".to_string(),
                icon: rsx!(Icon { icon: FaFileInvoiceDollar }),
            },

            IconName {
                name: "FaFileInvoice".to_string(),
                icon: rsx!(Icon { icon: FaFileInvoice }),
            },

            IconName {
                name: "FaFileLines".to_string(),
                icon: rsx!(Icon { icon: FaFileLines }),
            },

            IconName {
                name: "FaFileMedical".to_string(),
                icon: rsx!(Icon { icon: FaFileMedical }),
            },

            IconName {
                name: "FaFilePdf".to_string(),
                icon: rsx!(Icon { icon: FaFilePdf }),
            },

            IconName {
                name: "FaFilePen".to_string(),
                icon: rsx!(Icon { icon: FaFilePen }),
            },

            IconName {
                name: "FaFilePowerpoint".to_string(),
                icon: rsx!(Icon { icon: FaFilePowerpoint }),
            },

            IconName {
                name: "FaFilePrescription".to_string(),
                icon: rsx!(Icon { icon: FaFilePrescription }),
            },

            IconName {
                name: "FaFileShield".to_string(),
                icon: rsx!(Icon { icon: FaFileShield }),
            },

            IconName {
                name: "FaFileSignature".to_string(),
                icon: rsx!(Icon { icon: FaFileSignature }),
            },

            IconName {
                name: "FaFileVideo".to_string(),
                icon: rsx!(Icon { icon: FaFileVideo }),
            },

            IconName {
                name: "FaFileWaveform".to_string(),
                icon: rsx!(Icon { icon: FaFileWaveform }),
            },

            IconName {
                name: "FaFileWord".to_string(),
                icon: rsx!(Icon { icon: FaFileWord }),
            },

            IconName {
                name: "FaFileZipper".to_string(),
                icon: rsx!(Icon { icon: FaFileZipper }),
            },

            IconName {
                name: "FaFile".to_string(),
                icon: rsx!(Icon { icon: FaFile }),
            },

            IconName {
                name: "FaFillDrip".to_string(),
                icon: rsx!(Icon { icon: FaFillDrip }),
            },

            IconName {
                name: "FaFill".to_string(),
                icon: rsx!(Icon { icon: FaFill }),
            },

            IconName {
                name: "FaFilm".to_string(),
                icon: rsx!(Icon { icon: FaFilm }),
            },

            IconName {
                name: "FaFilterCircleDollar".to_string(),
                icon: rsx!(Icon { icon: FaFilterCircleDollar }),
            },

            IconName {
                name: "FaFilterCircleXmark".to_string(),
                icon: rsx!(Icon { icon: FaFilterCircleXmark }),
            },

            IconName {
                name: "FaFilter".to_string(),
                icon: rsx!(Icon { icon: FaFilter }),
            },

            IconName {
                name: "FaFingerprint".to_string(),
                icon: rsx!(Icon { icon: FaFingerprint }),
            },

            IconName {
                name: "FaFireBurner".to_string(),
                icon: rsx!(Icon { icon: FaFireBurner }),
            },

            IconName {
                name: "FaFireExtinguisher".to_string(),
                icon: rsx!(Icon { icon: FaFireExtinguisher }),
            },

            IconName {
                name: "FaFireFlameCurved".to_string(),
                icon: rsx!(Icon { icon: FaFireFlameCurved }),
            },

            IconName {
                name: "FaFireFlameSimple".to_string(),
                icon: rsx!(Icon { icon: FaFireFlameSimple }),
            },

            IconName {
                name: "FaFire".to_string(),
                icon: rsx!(Icon { icon: FaFire }),
            },

            IconName {
                name: "FaFishFins".to_string(),
                icon: rsx!(Icon { icon: FaFishFins }),
            },

            IconName {
                name: "FaFish".to_string(),
                icon: rsx!(Icon { icon: FaFish }),
            },

            IconName {
                name: "FaFlagCheckered".to_string(),
                icon: rsx!(Icon { icon: FaFlagCheckered }),
            },

            IconName {
                name: "FaFlagUsa".to_string(),
                icon: rsx!(Icon { icon: FaFlagUsa }),
            },

            IconName {
                name: "FaFlag".to_string(),
                icon: rsx!(Icon { icon: FaFlag }),
            },

            IconName {
                name: "FaFlaskVial".to_string(),
                icon: rsx!(Icon { icon: FaFlaskVial }),
            },

            IconName {
                name: "FaFlask".to_string(),
                icon: rsx!(Icon { icon: FaFlask }),
            },

            IconName {
                name: "FaFloppyDisk".to_string(),
                icon: rsx!(Icon { icon: FaFloppyDisk }),
            },

            IconName {
                name: "FaFlorinSign".to_string(),
                icon: rsx!(Icon { icon: FaFlorinSign }),
            },

            IconName {
                name: "FaFolderClosed".to_string(),
                icon: rsx!(Icon { icon: FaFolderClosed }),
            },

            IconName {
                name: "FaFolderMinus".to_string(),
                icon: rsx!(Icon { icon: FaFolderMinus }),
            },

            IconName {
                name: "FaFolderOpen".to_string(),
                icon: rsx!(Icon { icon: FaFolderOpen }),
            },

            IconName {
                name: "FaFolderPlus".to_string(),
                icon: rsx!(Icon { icon: FaFolderPlus }),
            },

            IconName {
                name: "FaFolderTree".to_string(),
                icon: rsx!(Icon { icon: FaFolderTree }),
            },

            IconName {
                name: "FaFolder".to_string(),
                icon: rsx!(Icon { icon: FaFolder }),
            },

            IconName {
                name: "FaFontAwesome".to_string(),
                icon: rsx!(Icon { icon: FaFontAwesome }),
            },

            IconName {
                name: "FaFont".to_string(),
                icon: rsx!(Icon { icon: FaFont }),
            },

            IconName {
                name: "FaFootball".to_string(),
                icon: rsx!(Icon { icon: FaFootball }),
            },

            IconName {
                name: "FaForwardFast".to_string(),
                icon: rsx!(Icon { icon: FaForwardFast }),
            },

            IconName {
                name: "FaForwardStep".to_string(),
                icon: rsx!(Icon { icon: FaForwardStep }),
            },

            IconName {
                name: "FaForward".to_string(),
                icon: rsx!(Icon { icon: FaForward }),
            },

            IconName {
                name: "FaFrancSign".to_string(),
                icon: rsx!(Icon { icon: FaFrancSign }),
            },

            IconName {
                name: "FaFrog".to_string(),
                icon: rsx!(Icon { icon: FaFrog }),
            },

            IconName {
                name: "FaFutbol".to_string(),
                icon: rsx!(Icon { icon: FaFutbol }),
            },

            IconName {
                name: "FaG".to_string(),
                icon: rsx!(Icon { icon: FaG }),
            },

            IconName {
                name: "FaGamepad".to_string(),
                icon: rsx!(Icon { icon: FaGamepad }),
            },

            IconName {
                name: "FaGasPump".to_string(),
                icon: rsx!(Icon { icon: FaGasPump }),
            },

            IconName {
                name: "FaGaugeHigh".to_string(),
                icon: rsx!(Icon { icon: FaGaugeHigh }),
            },

            IconName {
                name: "FaGaugeSimpleHigh".to_string(),
                icon: rsx!(Icon { icon: FaGaugeSimpleHigh }),
            },

            IconName {
                name: "FaGaugeSimple".to_string(),
                icon: rsx!(Icon { icon: FaGaugeSimple }),
            },

            IconName {
                name: "FaGauge".to_string(),
                icon: rsx!(Icon { icon: FaGauge }),
            },

            IconName {
                name: "FaGavel".to_string(),
                icon: rsx!(Icon { icon: FaGavel }),
            },

            IconName {
                name: "FaGear".to_string(),
                icon: rsx!(Icon { icon: FaGear }),
            },

            IconName {
                name: "FaGears".to_string(),
                icon: rsx!(Icon { icon: FaGears }),
            },

            IconName {
                name: "FaGem".to_string(),
                icon: rsx!(Icon { icon: FaGem }),
            },

            IconName {
                name: "FaGenderless".to_string(),
                icon: rsx!(Icon { icon: FaGenderless }),
            },

            IconName {
                name: "FaGhost".to_string(),
                icon: rsx!(Icon { icon: FaGhost }),
            },

            IconName {
                name: "FaGift".to_string(),
                icon: rsx!(Icon { icon: FaGift }),
            },

            IconName {
                name: "FaGifts".to_string(),
                icon: rsx!(Icon { icon: FaGifts }),
            },

            IconName {
                name: "FaGlassWaterDroplet".to_string(),
                icon: rsx!(Icon { icon: FaGlassWaterDroplet }),
            },

            IconName {
                name: "FaGlassWater".to_string(),
                icon: rsx!(Icon { icon: FaGlassWater }),
            },

            IconName {
                name: "FaGlasses".to_string(),
                icon: rsx!(Icon { icon: FaGlasses }),
            },

            IconName {
                name: "FaGlobe".to_string(),
                icon: rsx!(Icon { icon: FaGlobe }),
            },

            IconName {
                name: "FaGolfBallTee".to_string(),
                icon: rsx!(Icon { icon: FaGolfBallTee }),
            },

            IconName {
                name: "FaGopuram".to_string(),
                icon: rsx!(Icon { icon: FaGopuram }),
            },

            IconName {
                name: "FaGraduationCap".to_string(),
                icon: rsx!(Icon { icon: FaGraduationCap }),
            },

            IconName {
                name: "FaGreaterThanEqual".to_string(),
                icon: rsx!(Icon { icon: FaGreaterThanEqual }),
            },

            IconName {
                name: "FaGreaterThan".to_string(),
                icon: rsx!(Icon { icon: FaGreaterThan }),
            },

            IconName {
                name: "FaGripLinesVertical".to_string(),
                icon: rsx!(Icon { icon: FaGripLinesVertical }),
            },

            IconName {
                name: "FaGripLines".to_string(),
                icon: rsx!(Icon { icon: FaGripLines }),
            },

            IconName {
                name: "FaGripVertical".to_string(),
                icon: rsx!(Icon { icon: FaGripVertical }),
            },

            IconName {
                name: "FaGrip".to_string(),
                icon: rsx!(Icon { icon: FaGrip }),
            },

            IconName {
                name: "FaGroupArrowsRotate".to_string(),
                icon: rsx!(Icon { icon: FaGroupArrowsRotate }),
            },

            IconName {
                name: "FaGuaraniSign".to_string(),
                icon: rsx!(Icon { icon: FaGuaraniSign }),
            },

            IconName {
                name: "FaGuitar".to_string(),
                icon: rsx!(Icon { icon: FaGuitar }),
            },

            IconName {
                name: "FaGun".to_string(),
                icon: rsx!(Icon { icon: FaGun }),
            },

            IconName {
                name: "FaH".to_string(),
                icon: rsx!(Icon { icon: FaH }),
            },

            IconName {
                name: "FaHammer".to_string(),
                icon: rsx!(Icon { icon: FaHammer }),
            },

            IconName {
                name: "FaHamsa".to_string(),
                icon: rsx!(Icon { icon: FaHamsa }),
            },

            IconName {
                name: "FaHandBackFist".to_string(),
                icon: rsx!(Icon { icon: FaHandBackFist }),
            },

            IconName {
                name: "FaHandDots".to_string(),
                icon: rsx!(Icon { icon: FaHandDots }),
            },

            IconName {
                name: "FaHandFist".to_string(),
                icon: rsx!(Icon { icon: FaHandFist }),
            },

            IconName {
                name: "FaHandHoldingDollar".to_string(),
                icon: rsx!(Icon { icon: FaHandHoldingDollar }),
            },

            IconName {
                name: "FaHandHoldingDroplet".to_string(),
                icon: rsx!(Icon { icon: FaHandHoldingDroplet }),
            },

            IconName {
                name: "FaHandHoldingHand".to_string(),
                icon: rsx!(Icon { icon: FaHandHoldingHand }),
            },

            IconName {
                name: "FaHandHoldingHeart".to_string(),
                icon: rsx!(Icon { icon: FaHandHoldingHeart }),
            },

            IconName {
                name: "FaHandHoldingMedical".to_string(),
                icon: rsx!(Icon { icon: FaHandHoldingMedical }),
            },

            IconName {
                name: "FaHandHolding".to_string(),
                icon: rsx!(Icon { icon: FaHandHolding }),
            },

            IconName {
                name: "FaHandLizard".to_string(),
                icon: rsx!(Icon { icon: FaHandLizard }),
            },

            IconName {
                name: "FaHandMiddleFinger".to_string(),
                icon: rsx!(Icon { icon: FaHandMiddleFinger }),
            },

            IconName {
                name: "FaHandPeace".to_string(),
                icon: rsx!(Icon { icon: FaHandPeace }),
            },

            IconName {
                name: "FaHandPointDown".to_string(),
                icon: rsx!(Icon { icon: FaHandPointDown }),
            },

            IconName {
                name: "FaHandPointLeft".to_string(),
                icon: rsx!(Icon { icon: FaHandPointLeft }),
            },

            IconName {
                name: "FaHandPointRight".to_string(),
                icon: rsx!(Icon { icon: FaHandPointRight }),
            },

            IconName {
                name: "FaHandPointUp".to_string(),
                icon: rsx!(Icon { icon: FaHandPointUp }),
            },

            IconName {
                name: "FaHandPointer".to_string(),
                icon: rsx!(Icon { icon: FaHandPointer }),
            },

            IconName {
                name: "FaHandScissors".to_string(),
                icon: rsx!(Icon { icon: FaHandScissors }),
            },

            IconName {
                name: "FaHandSparkles".to_string(),
                icon: rsx!(Icon { icon: FaHandSparkles }),
            },

            IconName {
                name: "FaHandSpock".to_string(),
                icon: rsx!(Icon { icon: FaHandSpock }),
            },

            IconName {
                name: "FaHand".to_string(),
                icon: rsx!(Icon { icon: FaHand }),
            },

            IconName {
                name: "FaHandcuffs".to_string(),
                icon: rsx!(Icon { icon: FaHandcuffs }),
            },

            IconName {
                name: "FaHandsAslInterpreting".to_string(),
                icon: rsx!(Icon { icon: FaHandsAslInterpreting }),
            },

            IconName {
                name: "FaHandsBound".to_string(),
                icon: rsx!(Icon { icon: FaHandsBound }),
            },

            IconName {
                name: "FaHandsBubbles".to_string(),
                icon: rsx!(Icon { icon: FaHandsBubbles }),
            },

            IconName {
                name: "FaHandsClapping".to_string(),
                icon: rsx!(Icon { icon: FaHandsClapping }),
            },

            IconName {
                name: "FaHandsHoldingChild".to_string(),
                icon: rsx!(Icon { icon: FaHandsHoldingChild }),
            },

            IconName {
                name: "FaHandsHoldingCircle".to_string(),
                icon: rsx!(Icon { icon: FaHandsHoldingCircle }),
            },

            IconName {
                name: "FaHandsHolding".to_string(),
                icon: rsx!(Icon { icon: FaHandsHolding }),
            },

            IconName {
                name: "FaHandsPraying".to_string(),
                icon: rsx!(Icon { icon: FaHandsPraying }),
            },

            IconName {
                name: "FaHands".to_string(),
                icon: rsx!(Icon { icon: FaHands }),
            },

            IconName {
                name: "FaHandshakeAngle".to_string(),
                icon: rsx!(Icon { icon: FaHandshakeAngle }),
            },

            IconName {
                name: "FaHandshakeSimpleSlash".to_string(),
                icon: rsx!(Icon { icon: FaHandshakeSimpleSlash }),
            },

            IconName {
                name: "FaHandshakeSimple".to_string(),
                icon: rsx!(Icon { icon: FaHandshakeSimple }),
            },

            IconName {
                name: "FaHandshakeSlash".to_string(),
                icon: rsx!(Icon { icon: FaHandshakeSlash }),
            },

            IconName {
                name: "FaHandshake".to_string(),
                icon: rsx!(Icon { icon: FaHandshake }),
            },

            IconName {
                name: "FaHanukiah".to_string(),
                icon: rsx!(Icon { icon: FaHanukiah }),
            },

            IconName {
                name: "FaHardDrive".to_string(),
                icon: rsx!(Icon { icon: FaHardDrive }),
            },

            IconName {
                name: "FaHashtag".to_string(),
                icon: rsx!(Icon { icon: FaHashtag }),
            },

            IconName {
                name: "FaHatCowboySide".to_string(),
                icon: rsx!(Icon { icon: FaHatCowboySide }),
            },

            IconName {
                name: "FaHatCowboy".to_string(),
                icon: rsx!(Icon { icon: FaHatCowboy }),
            },

            IconName {
                name: "FaHatWizard".to_string(),
                icon: rsx!(Icon { icon: FaHatWizard }),
            },

            IconName {
                name: "FaHeadSideCoughSlash".to_string(),
                icon: rsx!(Icon { icon: FaHeadSideCoughSlash }),
            },

            IconName {
                name: "FaHeadSideCough".to_string(),
                icon: rsx!(Icon { icon: FaHeadSideCough }),
            },

            IconName {
                name: "FaHeadSideMask".to_string(),
                icon: rsx!(Icon { icon: FaHeadSideMask }),
            },

            IconName {
                name: "FaHeadSideVirus".to_string(),
                icon: rsx!(Icon { icon: FaHeadSideVirus }),
            },

            IconName {
                name: "FaHeading".to_string(),
                icon: rsx!(Icon { icon: FaHeading }),
            },

            IconName {
                name: "FaHeadphonesSimple".to_string(),
                icon: rsx!(Icon { icon: FaHeadphonesSimple }),
            },

            IconName {
                name: "FaHeadphones".to_string(),
                icon: rsx!(Icon { icon: FaHeadphones }),
            },

            IconName {
                name: "FaHeadset".to_string(),
                icon: rsx!(Icon { icon: FaHeadset }),
            },

            IconName {
                name: "FaHeartCircleBolt".to_string(),
                icon: rsx!(Icon { icon: FaHeartCircleBolt }),
            },

            IconName {
                name: "FaHeartCircleCheck".to_string(),
                icon: rsx!(Icon { icon: FaHeartCircleCheck }),
            },

            IconName {
                name: "FaHeartCircleExclamation".to_string(),
                icon: rsx!(Icon { icon: FaHeartCircleExclamation }),
            },

            IconName {
                name: "FaHeartCircleMinus".to_string(),
                icon: rsx!(Icon { icon: FaHeartCircleMinus }),
            },

            IconName {
                name: "FaHeartCirclePlus".to_string(),
                icon: rsx!(Icon { icon: FaHeartCirclePlus }),
            },

            IconName {
                name: "FaHeartCircleXmark".to_string(),
                icon: rsx!(Icon { icon: FaHeartCircleXmark }),
            },

            IconName {
                name: "FaHeartCrack".to_string(),
                icon: rsx!(Icon { icon: FaHeartCrack }),
            },

            IconName {
                name: "FaHeartPulse".to_string(),
                icon: rsx!(Icon { icon: FaHeartPulse }),
            },

            IconName {
                name: "FaHeart".to_string(),
                icon: rsx!(Icon { icon: FaHeart }),
            },

            IconName {
                name: "FaHelicopterSymbol".to_string(),
                icon: rsx!(Icon { icon: FaHelicopterSymbol }),
            },

            IconName {
                name: "FaHelicopter".to_string(),
                icon: rsx!(Icon { icon: FaHelicopter }),
            },

            IconName {
                name: "FaHelmetSafety".to_string(),
                icon: rsx!(Icon { icon: FaHelmetSafety }),
            },

            IconName {
                name: "FaHelmetUn".to_string(),
                icon: rsx!(Icon { icon: FaHelmetUn }),
            },

            IconName {
                name: "FaHighlighter".to_string(),
                icon: rsx!(Icon { icon: FaHighlighter }),
            },

            IconName {
                name: "FaHillAvalanche".to_string(),
                icon: rsx!(Icon { icon: FaHillAvalanche }),
            },

            IconName {
                name: "FaHillRockslide".to_string(),
                icon: rsx!(Icon { icon: FaHillRockslide }),
            },

            IconName {
                name: "FaHippo".to_string(),
                icon: rsx!(Icon { icon: FaHippo }),
            },

            IconName {
                name: "FaHockeyPuck".to_string(),
                icon: rsx!(Icon { icon: FaHockeyPuck }),
            },

            IconName {
                name: "FaHollyBerry".to_string(),
                icon: rsx!(Icon { icon: FaHollyBerry }),
            },

            IconName {
                name: "FaHorseHead".to_string(),
                icon: rsx!(Icon { icon: FaHorseHead }),
            },

            IconName {
                name: "FaHorse".to_string(),
                icon: rsx!(Icon { icon: FaHorse }),
            },

            IconName {
                name: "FaHospitalUser".to_string(),
                icon: rsx!(Icon { icon: FaHospitalUser }),
            },

            IconName {
                name: "FaHospital".to_string(),
                icon: rsx!(Icon { icon: FaHospital }),
            },

            IconName {
                name: "FaHotTubPerson".to_string(),
                icon: rsx!(Icon { icon: FaHotTubPerson }),
            },

            IconName {
                name: "FaHotdog".to_string(),
                icon: rsx!(Icon { icon: FaHotdog }),
            },

            IconName {
                name: "FaHotel".to_string(),
                icon: rsx!(Icon { icon: FaHotel }),
            },

            IconName {
                name: "FaHourglassEmpty".to_string(),
                icon: rsx!(Icon { icon: FaHourglassEmpty }),
            },

            IconName {
                name: "FaHourglassEnd".to_string(),
                icon: rsx!(Icon { icon: FaHourglassEnd }),
            },

            IconName {
                name: "FaHourglassStart".to_string(),
                icon: rsx!(Icon { icon: FaHourglassStart }),
            },

            IconName {
                name: "FaHourglass".to_string(),
                icon: rsx!(Icon { icon: FaHourglass }),
            },

            IconName {
                name: "FaHouseChimneyCrack".to_string(),
                icon: rsx!(Icon { icon: FaHouseChimneyCrack }),
            },

            IconName {
                name: "FaHouseChimneyMedical".to_string(),
                icon: rsx!(Icon { icon: FaHouseChimneyMedical }),
            },

            IconName {
                name: "FaHouseChimneyUser".to_string(),
                icon: rsx!(Icon { icon: FaHouseChimneyUser }),
            },

            IconName {
                name: "FaHouseChimneyWindow".to_string(),
                icon: rsx!(Icon { icon: FaHouseChimneyWindow }),
            },

            IconName {
                name: "FaHouseChimney".to_string(),
                icon: rsx!(Icon { icon: FaHouseChimney }),
            },

            IconName {
                name: "FaHouseCircleCheck".to_string(),
                icon: rsx!(Icon { icon: FaHouseCircleCheck }),
            },

            IconName {
                name: "FaHouseCircleExclamation".to_string(),
                icon: rsx!(Icon { icon: FaHouseCircleExclamation }),
            },

            IconName {
                name: "FaHouseCircleXmark".to_string(),
                icon: rsx!(Icon { icon: FaHouseCircleXmark }),
            },

            IconName {
                name: "FaHouseCrack".to_string(),
                icon: rsx!(Icon { icon: FaHouseCrack }),
            },

            IconName {
                name: "FaHouseFire".to_string(),
                icon: rsx!(Icon { icon: FaHouseFire }),
            },

            IconName {
                name: "FaHouseFlag".to_string(),
                icon: rsx!(Icon { icon: FaHouseFlag }),
            },

            IconName {
                name: "FaHouseFloodWaterCircleArrowRight".to_string(),
                icon: rsx!(Icon { icon: FaHouseFloodWaterCircleArrowRight }),
            },

            IconName {
                name: "FaHouseFloodWater".to_string(),
                icon: rsx!(Icon { icon: FaHouseFloodWater }),
            },

            IconName {
                name: "FaHouseLaptop".to_string(),
                icon: rsx!(Icon { icon: FaHouseLaptop }),
            },

            IconName {
                name: "FaHouseLock".to_string(),
                icon: rsx!(Icon { icon: FaHouseLock }),
            },

            IconName {
                name: "FaHouseMedicalCircleCheck".to_string(),
                icon: rsx!(Icon { icon: FaHouseMedicalCircleCheck }),
            },

            IconName {
                name: "FaHouseMedicalCircleExclamation".to_string(),
                icon: rsx!(Icon { icon: FaHouseMedicalCircleExclamation }),
            },

            IconName {
                name: "FaHouseMedicalCircleXmark".to_string(),
                icon: rsx!(Icon { icon: FaHouseMedicalCircleXmark }),
            },

            IconName {
                name: "FaHouseMedicalFlag".to_string(),
                icon: rsx!(Icon { icon: FaHouseMedicalFlag }),
            },

            IconName {
                name: "FaHouseMedical".to_string(),
                icon: rsx!(Icon { icon: FaHouseMedical }),
            },

            IconName {
                name: "FaHouseSignal".to_string(),
                icon: rsx!(Icon { icon: FaHouseSignal }),
            },

            IconName {
                name: "FaHouseTsunami".to_string(),
                icon: rsx!(Icon { icon: FaHouseTsunami }),
            },

            IconName {
                name: "FaHouseUser".to_string(),
                icon: rsx!(Icon { icon: FaHouseUser }),
            },

            IconName {
                name: "FaHouse".to_string(),
                icon: rsx!(Icon { icon: FaHouse }),
            },

            IconName {
                name: "FaHryvniaSign".to_string(),
                icon: rsx!(Icon { icon: FaHryvniaSign }),
            },

            IconName {
                name: "FaHurricane".to_string(),
                icon: rsx!(Icon { icon: FaHurricane }),
            },

            IconName {
                name: "FaICursor".to_string(),
                icon: rsx!(Icon { icon: FaICursor }),
            },

            IconName {
                name: "FaI".to_string(),
                icon: rsx!(Icon { icon: FaI }),
            },

            IconName {
                name: "FaIceCream".to_string(),
                icon: rsx!(Icon { icon: FaIceCream }),
            },

            IconName {
                name: "FaIcicles".to_string(),
                icon: rsx!(Icon { icon: FaIcicles }),
            },

            IconName {
                name: "FaIcons".to_string(),
                icon: rsx!(Icon { icon: FaIcons }),
            },

            IconName {
                name: "FaIdBadge".to_string(),
                icon: rsx!(Icon { icon: FaIdBadge }),
            },

            IconName {
                name: "FaIdCardClip".to_string(),
                icon: rsx!(Icon { icon: FaIdCardClip }),
            },

            IconName {
                name: "FaIdCard".to_string(),
                icon: rsx!(Icon { icon: FaIdCard }),
            },

            IconName {
                name: "FaIgloo".to_string(),
                icon: rsx!(Icon { icon: FaIgloo }),
            },

            IconName {
                name: "FaImagePortrait".to_string(),
                icon: rsx!(Icon { icon: FaImagePortrait }),
            },

            IconName {
                name: "FaImage".to_string(),
                icon: rsx!(Icon { icon: FaImage }),
            },

            IconName {
                name: "FaImages".to_string(),
                icon: rsx!(Icon { icon: FaImages }),
            },

            IconName {
                name: "FaInbox".to_string(),
                icon: rsx!(Icon { icon: FaInbox }),
            },

            IconName {
                name: "FaIndent".to_string(),
                icon: rsx!(Icon { icon: FaIndent }),
            },

            IconName {
                name: "FaIndianRupeeSign".to_string(),
                icon: rsx!(Icon { icon: FaIndianRupeeSign }),
            },

            IconName {
                name: "FaIndustry".to_string(),
                icon: rsx!(Icon { icon: FaIndustry }),
            },

            IconName {
                name: "FaInfinity".to_string(),
                icon: rsx!(Icon { icon: FaInfinity }),
            },

            IconName {
                name: "FaInfo".to_string(),
                icon: rsx!(Icon { icon: FaInfo }),
            },

            IconName {
                name: "FaItalic".to_string(),
                icon: rsx!(Icon { icon: FaItalic }),
            },

            IconName {
                name: "FaJ".to_string(),
                icon: rsx!(Icon { icon: FaJ }),
            },

            IconName {
                name: "FaJarWheat".to_string(),
                icon: rsx!(Icon { icon: FaJarWheat }),
            },

            IconName {
                name: "FaJar".to_string(),
                icon: rsx!(Icon { icon: FaJar }),
            },

            IconName {
                name: "FaJedi".to_string(),
                icon: rsx!(Icon { icon: FaJedi }),
            },

            IconName {
                name: "FaJetFighterUp".to_string(),
                icon: rsx!(Icon { icon: FaJetFighterUp }),
            },

            IconName {
                name: "FaJetFighter".to_string(),
                icon: rsx!(Icon { icon: FaJetFighter }),
            },

            IconName {
                name: "FaJoint".to_string(),
                icon: rsx!(Icon { icon: FaJoint }),
            },

            IconName {
                name: "FaJugDetergent".to_string(),
                icon: rsx!(Icon { icon: FaJugDetergent }),
            },

            IconName {
                name: "FaK".to_string(),
                icon: rsx!(Icon { icon: FaK }),
            },

            IconName {
                name: "FaKaaba".to_string(),
                icon: rsx!(Icon { icon: FaKaaba }),
            },

            IconName {
                name: "FaKey".to_string(),
                icon: rsx!(Icon { icon: FaKey }),
            },

            IconName {
                name: "FaKeyboard".to_string(),
                icon: rsx!(Icon { icon: FaKeyboard }),
            },

            IconName {
                name: "FaKhanda".to_string(),
                icon: rsx!(Icon { icon: FaKhanda }),
            },

            IconName {
                name: "FaKipSign".to_string(),
                icon: rsx!(Icon { icon: FaKipSign }),
            },

            IconName {
                name: "FaKitMedical".to_string(),
                icon: rsx!(Icon { icon: FaKitMedical }),
            },

            IconName {
                name: "FaKitchenSet".to_string(),
                icon: rsx!(Icon { icon: FaKitchenSet }),
            },

            IconName {
                name: "FaKiwiBird".to_string(),
                icon: rsx!(Icon { icon: FaKiwiBird }),
            },

            IconName {
                name: "FaL".to_string(),
                icon: rsx!(Icon { icon: FaL }),
            },

            IconName {
                name: "FaLandMineOn".to_string(),
                icon: rsx!(Icon { icon: FaLandMineOn }),
            },

            IconName {
                name: "FaLandmarkDome".to_string(),
                icon: rsx!(Icon { icon: FaLandmarkDome }),
            },

            IconName {
                name: "FaLandmarkFlag".to_string(),
                icon: rsx!(Icon { icon: FaLandmarkFlag }),
            },

            IconName {
                name: "FaLandmark".to_string(),
                icon: rsx!(Icon { icon: FaLandmark }),
            },

            IconName {
                name: "FaLanguage".to_string(),
                icon: rsx!(Icon { icon: FaLanguage }),
            },

            IconName {
                name: "FaLaptopCode".to_string(),
                icon: rsx!(Icon { icon: FaLaptopCode }),
            },

            IconName {
                name: "FaLaptopFile".to_string(),
                icon: rsx!(Icon { icon: FaLaptopFile }),
            },

            IconName {
                name: "FaLaptopMedical".to_string(),
                icon: rsx!(Icon { icon: FaLaptopMedical }),
            },

            IconName {
                name: "FaLaptop".to_string(),
                icon: rsx!(Icon { icon: FaLaptop }),
            },

            IconName {
                name: "FaLariSign".to_string(),
                icon: rsx!(Icon { icon: FaLariSign }),
            },

            IconName {
                name: "FaLayerGroup".to_string(),
                icon: rsx!(Icon { icon: FaLayerGroup }),
            },

            IconName {
                name: "FaLeaf".to_string(),
                icon: rsx!(Icon { icon: FaLeaf }),
            },

            IconName {
                name: "FaLeftLong".to_string(),
                icon: rsx!(Icon { icon: FaLeftLong }),
            },

            IconName {
                name: "FaLeftRight".to_string(),
                icon: rsx!(Icon { icon: FaLeftRight }),
            },

            IconName {
                name: "FaLemon".to_string(),
                icon: rsx!(Icon { icon: FaLemon }),
            },

            IconName {
                name: "FaLessThanEqual".to_string(),
                icon: rsx!(Icon { icon: FaLessThanEqual }),
            },

            IconName {
                name: "FaLessThan".to_string(),
                icon: rsx!(Icon { icon: FaLessThan }),
            },

            IconName {
                name: "FaLifeRing".to_string(),
                icon: rsx!(Icon { icon: FaLifeRing }),
            },

            IconName {
                name: "FaLightbulb".to_string(),
                icon: rsx!(Icon { icon: FaLightbulb }),
            },

            IconName {
                name: "FaLinesLeaning".to_string(),
                icon: rsx!(Icon { icon: FaLinesLeaning }),
            },

            IconName {
                name: "FaLinkSlash".to_string(),
                icon: rsx!(Icon { icon: FaLinkSlash }),
            },

            IconName {
                name: "FaLink".to_string(),
                icon: rsx!(Icon { icon: FaLink }),
            },

            IconName {
                name: "FaLiraSign".to_string(),
                icon: rsx!(Icon { icon: FaLiraSign }),
            },

            IconName {
                name: "FaListCheck".to_string(),
                icon: rsx!(Icon { icon: FaListCheck }),
            },

            IconName {
                name: "FaListOl".to_string(),
                icon: rsx!(Icon { icon: FaListOl }),
            },

            IconName {
                name: "FaListUl".to_string(),
                icon: rsx!(Icon { icon: FaListUl }),
            },

            IconName {
                name: "FaList".to_string(),
                icon: rsx!(Icon { icon: FaList }),
            },

            IconName {
                name: "FaLitecoinSign".to_string(),
                icon: rsx!(Icon { icon: FaLitecoinSign }),
            },

            IconName {
                name: "FaLocationArrow".to_string(),
                icon: rsx!(Icon { icon: FaLocationArrow }),
            },

            IconName {
                name: "FaLocationCrosshairs".to_string(),
                icon: rsx!(Icon { icon: FaLocationCrosshairs }),
            },

            IconName {
                name: "FaLocationDot".to_string(),
                icon: rsx!(Icon { icon: FaLocationDot }),
            },

            IconName {
                name: "FaLocationPinLock".to_string(),
                icon: rsx!(Icon { icon: FaLocationPinLock }),
            },

            IconName {
                name: "FaLocationPin".to_string(),
                icon: rsx!(Icon { icon: FaLocationPin }),
            },

            IconName {
                name: "FaLockOpen".to_string(),
                icon: rsx!(Icon { icon: FaLockOpen }),
            },

            IconName {
                name: "FaLock".to_string(),
                icon: rsx!(Icon { icon: FaLock }),
            },

            IconName {
                name: "FaLocust".to_string(),
                icon: rsx!(Icon { icon: FaLocust }),
            },

            IconName {
                name: "FaLungsVirus".to_string(),
                icon: rsx!(Icon { icon: FaLungsVirus }),
            },

            IconName {
                name: "FaLungs".to_string(),
                icon: rsx!(Icon { icon: FaLungs }),
            },

            IconName {
                name: "FaM".to_string(),
                icon: rsx!(Icon { icon: FaM }),
            },

            IconName {
                name: "FaMagnet".to_string(),
                icon: rsx!(Icon { icon: FaMagnet }),
            },

            IconName {
                name: "FaMagnifyingGlassArrowRight".to_string(),
                icon: rsx!(Icon { icon: FaMagnifyingGlassArrowRight }),
            },

            IconName {
                name: "FaMagnifyingGlassChart".to_string(),
                icon: rsx!(Icon { icon: FaMagnifyingGlassChart }),
            },

            IconName {
                name: "FaMagnifyingGlassDollar".to_string(),
                icon: rsx!(Icon { icon: FaMagnifyingGlassDollar }),
            },

            IconName {
                name: "FaMagnifyingGlassLocation".to_string(),
                icon: rsx!(Icon { icon: FaMagnifyingGlassLocation }),
            },

            IconName {
                name: "FaMagnifyingGlassMinus".to_string(),
                icon: rsx!(Icon { icon: FaMagnifyingGlassMinus }),
            },

            IconName {
                name: "FaMagnifyingGlassPlus".to_string(),
                icon: rsx!(Icon { icon: FaMagnifyingGlassPlus }),
            },

            IconName {
                name: "FaMagnifyingGlass".to_string(),
                icon: rsx!(Icon { icon: FaMagnifyingGlass }),
            },

            IconName {
                name: "FaManatSign".to_string(),
                icon: rsx!(Icon { icon: FaManatSign }),
            },

            IconName {
                name: "FaMapLocationDot".to_string(),
                icon: rsx!(Icon { icon: FaMapLocationDot }),
            },

            IconName {
                name: "FaMapLocation".to_string(),
                icon: rsx!(Icon { icon: FaMapLocation }),
            },

            IconName {
                name: "FaMapPin".to_string(),
                icon: rsx!(Icon { icon: FaMapPin }),
            },

            IconName {
                name: "FaMap".to_string(),
                icon: rsx!(Icon { icon: FaMap }),
            },

            IconName {
                name: "FaMarker".to_string(),
                icon: rsx!(Icon { icon: FaMarker }),
            },

            IconName {
                name: "FaMarsAndVenusBurst".to_string(),
                icon: rsx!(Icon { icon: FaMarsAndVenusBurst }),
            },

            IconName {
                name: "FaMarsAndVenus".to_string(),
                icon: rsx!(Icon { icon: FaMarsAndVenus }),
            },

            IconName {
                name: "FaMarsDouble".to_string(),
                icon: rsx!(Icon { icon: FaMarsDouble }),
            },

            IconName {
                name: "FaMarsStrokeRight".to_string(),
                icon: rsx!(Icon { icon: FaMarsStrokeRight }),
            },

            IconName {
                name: "FaMarsStrokeUp".to_string(),
                icon: rsx!(Icon { icon: FaMarsStrokeUp }),
            },

            IconName {
                name: "FaMarsStroke".to_string(),
                icon: rsx!(Icon { icon: FaMarsStroke }),
            },

            IconName {
                name: "FaMars".to_string(),
                icon: rsx!(Icon { icon: FaMars }),
            },

            IconName {
                name: "FaMartiniGlassCitrus".to_string(),
                icon: rsx!(Icon { icon: FaMartiniGlassCitrus }),
            },

            IconName {
                name: "FaMartiniGlassEmpty".to_string(),
                icon: rsx!(Icon { icon: FaMartiniGlassEmpty }),
            },

            IconName {
                name: "FaMartiniGlass".to_string(),
                icon: rsx!(Icon { icon: FaMartiniGlass }),
            },

            IconName {
                name: "FaMaskFace".to_string(),
                icon: rsx!(Icon { icon: FaMaskFace }),
            },

            IconName {
                name: "FaMaskVentilator".to_string(),
                icon: rsx!(Icon { icon: FaMaskVentilator }),
            },

            IconName {
                name: "FaMask".to_string(),
                icon: rsx!(Icon { icon: FaMask }),
            },

            IconName {
                name: "FaMasksTheater".to_string(),
                icon: rsx!(Icon { icon: FaMasksTheater }),
            },

            IconName {
                name: "FaMattressPillow".to_string(),
                icon: rsx!(Icon { icon: FaMattressPillow }),
            },

            IconName {
                name: "FaMaximize".to_string(),
                icon: rsx!(Icon { icon: FaMaximize }),
            },

            IconName {
                name: "FaMedal".to_string(),
                icon: rsx!(Icon { icon: FaMedal }),
            },

            IconName {
                name: "FaMemory".to_string(),
                icon: rsx!(Icon { icon: FaMemory }),
            },

            IconName {
                name: "FaMenorah".to_string(),
                icon: rsx!(Icon { icon: FaMenorah }),
            },

            IconName {
                name: "FaMercury".to_string(),
                icon: rsx!(Icon { icon: FaMercury }),
            },

            IconName {
                name: "FaMessage".to_string(),
                icon: rsx!(Icon { icon: FaMessage }),
            },

            IconName {
                name: "FaMeteor".to_string(),
                icon: rsx!(Icon { icon: FaMeteor }),
            },

            IconName {
                name: "FaMicrochip".to_string(),
                icon: rsx!(Icon { icon: FaMicrochip }),
            },

            IconName {
                name: "FaMicrophoneLinesSlash".to_string(),
                icon: rsx!(Icon { icon: FaMicrophoneLinesSlash }),
            },

            IconName {
                name: "FaMicrophoneLines".to_string(),
                icon: rsx!(Icon { icon: FaMicrophoneLines }),
            },

            IconName {
                name: "FaMicrophoneSlash".to_string(),
                icon: rsx!(Icon { icon: FaMicrophoneSlash }),
            },

            IconName {
                name: "FaMicrophone".to_string(),
                icon: rsx!(Icon { icon: FaMicrophone }),
            },

            IconName {
                name: "FaMicroscope".to_string(),
                icon: rsx!(Icon { icon: FaMicroscope }),
            },

            IconName {
                name: "FaMillSign".to_string(),
                icon: rsx!(Icon { icon: FaMillSign }),
            },

            IconName {
                name: "FaMinimize".to_string(),
                icon: rsx!(Icon { icon: FaMinimize }),
            },

            IconName {
                name: "FaMinus".to_string(),
                icon: rsx!(Icon { icon: FaMinus }),
            },

            IconName {
                name: "FaMitten".to_string(),
                icon: rsx!(Icon { icon: FaMitten }),
            },

            IconName {
                name: "FaMobileButton".to_string(),
                icon: rsx!(Icon { icon: FaMobileButton }),
            },

            IconName {
                name: "FaMobileRetro".to_string(),
                icon: rsx!(Icon { icon: FaMobileRetro }),
            },

            IconName {
                name: "FaMobileScreenButton".to_string(),
                icon: rsx!(Icon { icon: FaMobileScreenButton }),
            },

            IconName {
                name: "FaMobileScreen".to_string(),
                icon: rsx!(Icon { icon: FaMobileScreen }),
            },

            IconName {
                name: "FaMobile".to_string(),
                icon: rsx!(Icon { icon: FaMobile }),
            },

            IconName {
                name: "FaMoneyBill1Wave".to_string(),
                icon: rsx!(Icon { icon: FaMoneyBill1Wave }),
            },

            IconName {
                name: "FaMoneyBill1".to_string(),
                icon: rsx!(Icon { icon: FaMoneyBill1 }),
            },

            IconName {
                name: "FaMoneyBillTransfer".to_string(),
                icon: rsx!(Icon { icon: FaMoneyBillTransfer }),
            },

            IconName {
                name: "FaMoneyBillTrendUp".to_string(),
                icon: rsx!(Icon { icon: FaMoneyBillTrendUp }),
            },

            IconName {
                name: "FaMoneyBillWave".to_string(),
                icon: rsx!(Icon { icon: FaMoneyBillWave }),
            },

            IconName {
                name: "FaMoneyBillWheat".to_string(),
                icon: rsx!(Icon { icon: FaMoneyBillWheat }),
            },

            IconName {
                name: "FaMoneyBill".to_string(),
                icon: rsx!(Icon { icon: FaMoneyBill }),
            },

            IconName {
                name: "FaMoneyBills".to_string(),
                icon: rsx!(Icon { icon: FaMoneyBills }),
            },

            IconName {
                name: "FaMoneyCheckDollar".to_string(),
                icon: rsx!(Icon { icon: FaMoneyCheckDollar }),
            },

            IconName {
                name: "FaMoneyCheck".to_string(),
                icon: rsx!(Icon { icon: FaMoneyCheck }),
            },

            IconName {
                name: "FaMonument".to_string(),
                icon: rsx!(Icon { icon: FaMonument }),
            },

            IconName {
                name: "FaMoon".to_string(),
                icon: rsx!(Icon { icon: FaMoon }),
            },

            IconName {
                name: "FaMortarPestle".to_string(),
                icon: rsx!(Icon { icon: FaMortarPestle }),
            },

            IconName {
                name: "FaMosque".to_string(),
                icon: rsx!(Icon { icon: FaMosque }),
            },

            IconName {
                name: "FaMosquitoNet".to_string(),
                icon: rsx!(Icon { icon: FaMosquitoNet }),
            },

            IconName {
                name: "FaMosquito".to_string(),
                icon: rsx!(Icon { icon: FaMosquito }),
            },

            IconName {
                name: "FaMotorcycle".to_string(),
                icon: rsx!(Icon { icon: FaMotorcycle }),
            },

            IconName {
                name: "FaMound".to_string(),
                icon: rsx!(Icon { icon: FaMound }),
            },

            IconName {
                name: "FaMountainCity".to_string(),
                icon: rsx!(Icon { icon: FaMountainCity }),
            },

            IconName {
                name: "FaMountainSun".to_string(),
                icon: rsx!(Icon { icon: FaMountainSun }),
            },

            IconName {
                name: "FaMountain".to_string(),
                icon: rsx!(Icon { icon: FaMountain }),
            },

            IconName {
                name: "FaMugHot".to_string(),
                icon: rsx!(Icon { icon: FaMugHot }),
            },

            IconName {
                name: "FaMugSaucer".to_string(),
                icon: rsx!(Icon { icon: FaMugSaucer }),
            },

            IconName {
                name: "FaMusic".to_string(),
                icon: rsx!(Icon { icon: FaMusic }),
            },

            IconName {
                name: "FaN".to_string(),
                icon: rsx!(Icon { icon: FaN }),
            },

            IconName {
                name: "FaNairaSign".to_string(),
                icon: rsx!(Icon { icon: FaNairaSign }),
            },

            IconName {
                name: "FaNetworkWired".to_string(),
                icon: rsx!(Icon { icon: FaNetworkWired }),
            },

            IconName {
                name: "FaNeuter".to_string(),
                icon: rsx!(Icon { icon: FaNeuter }),
            },

            IconName {
                name: "FaNewspaper".to_string(),
                icon: rsx!(Icon { icon: FaNewspaper }),
            },

            IconName {
                name: "FaNotEqual".to_string(),
                icon: rsx!(Icon { icon: FaNotEqual }),
            },

            IconName {
                name: "FaNoteSticky".to_string(),
                icon: rsx!(Icon { icon: FaNoteSticky }),
            },

            IconName {
                name: "FaNotesMedical".to_string(),
                icon: rsx!(Icon { icon: FaNotesMedical }),
            },

            IconName {
                name: "FaO".to_string(),
                icon: rsx!(Icon { icon: FaO }),
            },

            IconName {
                name: "FaObjectGroup".to_string(),
                icon: rsx!(Icon { icon: FaObjectGroup }),
            },

            IconName {
                name: "FaObjectUngroup".to_string(),
                icon: rsx!(Icon { icon: FaObjectUngroup }),
            },

            IconName {
                name: "FaOilCan".to_string(),
                icon: rsx!(Icon { icon: FaOilCan }),
            },

            IconName {
                name: "FaOilWell".to_string(),
                icon: rsx!(Icon { icon: FaOilWell }),
            },

            IconName {
                name: "FaOm".to_string(),
                icon: rsx!(Icon { icon: FaOm }),
            },

            IconName {
                name: "FaOtter".to_string(),
                icon: rsx!(Icon { icon: FaOtter }),
            },

            IconName {
                name: "FaOutdent".to_string(),
                icon: rsx!(Icon { icon: FaOutdent }),
            },

            IconName {
                name: "FaP".to_string(),
                icon: rsx!(Icon { icon: FaP }),
            },

            IconName {
                name: "FaPager".to_string(),
                icon: rsx!(Icon { icon: FaPager }),
            },

            IconName {
                name: "FaPaintRoller".to_string(),
                icon: rsx!(Icon { icon: FaPaintRoller }),
            },

            IconName {
                name: "FaPaintbrush".to_string(),
                icon: rsx!(Icon { icon: FaPaintbrush }),
            },

            IconName {
                name: "FaPalette".to_string(),
                icon: rsx!(Icon { icon: FaPalette }),
            },

            IconName {
                name: "FaPallet".to_string(),
                icon: rsx!(Icon { icon: FaPallet }),
            },

            IconName {
                name: "FaPanorama".to_string(),
                icon: rsx!(Icon { icon: FaPanorama }),
            },

            IconName {
                name: "FaPaperPlane".to_string(),
                icon: rsx!(Icon { icon: FaPaperPlane }),
            },

            IconName {
                name: "FaPaperclip".to_string(),
                icon: rsx!(Icon { icon: FaPaperclip }),
            },

            IconName {
                name: "FaParachuteBox".to_string(),
                icon: rsx!(Icon { icon: FaParachuteBox }),
            },

            IconName {
                name: "FaParagraph".to_string(),
                icon: rsx!(Icon { icon: FaParagraph }),
            },

            IconName {
                name: "FaPassport".to_string(),
                icon: rsx!(Icon { icon: FaPassport }),
            },

            IconName {
                name: "FaPaste".to_string(),
                icon: rsx!(Icon { icon: FaPaste }),
            },

            IconName {
                name: "FaPause".to_string(),
                icon: rsx!(Icon { icon: FaPause }),
            },

            IconName {
                name: "FaPaw".to_string(),
                icon: rsx!(Icon { icon: FaPaw }),
            },

            IconName {
                name: "FaPeace".to_string(),
                icon: rsx!(Icon { icon: FaPeace }),
            },

            IconName {
                name: "FaPenClip".to_string(),
                icon: rsx!(Icon { icon: FaPenClip }),
            },

            IconName {
                name: "FaPenFancy".to_string(),
                icon: rsx!(Icon { icon: FaPenFancy }),
            },

            IconName {
                name: "FaPenNib".to_string(),
                icon: rsx!(Icon { icon: FaPenNib }),
            },

            IconName {
                name: "FaPenRuler".to_string(),
                icon: rsx!(Icon { icon: FaPenRuler }),
            },

            IconName {
                name: "FaPenToSquare".to_string(),
                icon: rsx!(Icon { icon: FaPenToSquare }),
            },

            IconName {
                name: "FaPen".to_string(),
                icon: rsx!(Icon { icon: FaPen }),
            },

            IconName {
                name: "FaPencil".to_string(),
                icon: rsx!(Icon { icon: FaPencil }),
            },

            IconName {
                name: "FaPeopleArrowsLeftRight".to_string(),
                icon: rsx!(Icon { icon: FaPeopleArrowsLeftRight }),
            },

            IconName {
                name: "FaPeopleCarryBox".to_string(),
                icon: rsx!(Icon { icon: FaPeopleCarryBox }),
            },

            IconName {
                name: "FaPeopleGroup".to_string(),
                icon: rsx!(Icon { icon: FaPeopleGroup }),
            },

            IconName {
                name: "FaPeopleLine".to_string(),
                icon: rsx!(Icon { icon: FaPeopleLine }),
            },

            IconName {
                name: "FaPeoplePulling".to_string(),
                icon: rsx!(Icon { icon: FaPeoplePulling }),
            },

            IconName {
                name: "FaPeopleRobbery".to_string(),
                icon: rsx!(Icon { icon: FaPeopleRobbery }),
            },

            IconName {
                name: "FaPeopleRoof".to_string(),
                icon: rsx!(Icon { icon: FaPeopleRoof }),
            },

            IconName {
                name: "FaPepperHot".to_string(),
                icon: rsx!(Icon { icon: FaPepperHot }),
            },

            IconName {
                name: "FaPercent".to_string(),
                icon: rsx!(Icon { icon: FaPercent }),
            },

            IconName {
                name: "FaPersonArrowDownToLine".to_string(),
                icon: rsx!(Icon { icon: FaPersonArrowDownToLine }),
            },

            IconName {
                name: "FaPersonArrowUpFromLine".to_string(),
                icon: rsx!(Icon { icon: FaPersonArrowUpFromLine }),
            },

            IconName {
                name: "FaPersonBiking".to_string(),
                icon: rsx!(Icon { icon: FaPersonBiking }),
            },

            IconName {
                name: "FaPersonBooth".to_string(),
                icon: rsx!(Icon { icon: FaPersonBooth }),
            },

            IconName {
                name: "FaPersonBreastfeeding".to_string(),
                icon: rsx!(Icon { icon: FaPersonBreastfeeding }),
            },

            IconName {
                name: "FaPersonBurst".to_string(),
                icon: rsx!(Icon { icon: FaPersonBurst }),
            },

            IconName {
                name: "FaPersonCane".to_string(),
                icon: rsx!(Icon { icon: FaPersonCane }),
            },

            IconName {
                name: "FaPersonChalkboard".to_string(),
                icon: rsx!(Icon { icon: FaPersonChalkboard }),
            },

            IconName {
                name: "FaPersonCircleCheck".to_string(),
                icon: rsx!(Icon { icon: FaPersonCircleCheck }),
            },

            IconName {
                name: "FaPersonCircleExclamation".to_string(),
                icon: rsx!(Icon { icon: FaPersonCircleExclamation }),
            },

            IconName {
                name: "FaPersonCircleMinus".to_string(),
                icon: rsx!(Icon { icon: FaPersonCircleMinus }),
            },

            IconName {
                name: "FaPersonCirclePlus".to_string(),
                icon: rsx!(Icon { icon: FaPersonCirclePlus }),
            },

            IconName {
                name: "FaPersonCircleQuestion".to_string(),
                icon: rsx!(Icon { icon: FaPersonCircleQuestion }),
            },

            IconName {
                name: "FaPersonCircleXmark".to_string(),
                icon: rsx!(Icon { icon: FaPersonCircleXmark }),
            },

            IconName {
                name: "FaPersonDigging".to_string(),
                icon: rsx!(Icon { icon: FaPersonDigging }),
            },

            IconName {
                name: "FaPersonDotsFromLine".to_string(),
                icon: rsx!(Icon { icon: FaPersonDotsFromLine }),
            },

            IconName {
                name: "FaPersonDressBurst".to_string(),
                icon: rsx!(Icon { icon: FaPersonDressBurst }),
            },

            IconName {
                name: "FaPersonDress".to_string(),
                icon: rsx!(Icon { icon: FaPersonDress }),
            },

            IconName {
                name: "FaPersonDrowning".to_string(),
                icon: rsx!(Icon { icon: FaPersonDrowning }),
            },

            IconName {
                name: "FaPersonFallingBurst".to_string(),
                icon: rsx!(Icon { icon: FaPersonFallingBurst }),
            },

            IconName {
                name: "FaPersonFalling".to_string(),
                icon: rsx!(Icon { icon: FaPersonFalling }),
            },

            IconName {
                name: "FaPersonHalfDress".to_string(),
                icon: rsx!(Icon { icon: FaPersonHalfDress }),
            },

            IconName {
                name: "FaPersonHarassing".to_string(),
                icon: rsx!(Icon { icon: FaPersonHarassing }),
            },

            IconName {
                name: "FaPersonHiking".to_string(),
                icon: rsx!(Icon { icon: FaPersonHiking }),
            },

            IconName {
                name: "FaPersonMilitaryPointing".to_string(),
                icon: rsx!(Icon { icon: FaPersonMilitaryPointing }),
            },

            IconName {
                name: "FaPersonMilitaryRifle".to_string(),
                icon: rsx!(Icon { icon: FaPersonMilitaryRifle }),
            },

            IconName {
                name: "FaPersonMilitaryToPerson".to_string(),
                icon: rsx!(Icon { icon: FaPersonMilitaryToPerson }),
            },

            IconName {
                name: "FaPersonPraying".to_string(),
                icon: rsx!(Icon { icon: FaPersonPraying }),
            },

            IconName {
                name: "FaPersonPregnant".to_string(),
                icon: rsx!(Icon { icon: FaPersonPregnant }),
            },

            IconName {
                name: "FaPersonRays".to_string(),
                icon: rsx!(Icon { icon: FaPersonRays }),
            },

            IconName {
                name: "FaPersonRifle".to_string(),
                icon: rsx!(Icon { icon: FaPersonRifle }),
            },

            IconName {
                name: "FaPersonRunning".to_string(),
                icon: rsx!(Icon { icon: FaPersonRunning }),
            },

            IconName {
                name: "FaPersonShelter".to_string(),
                icon: rsx!(Icon { icon: FaPersonShelter }),
            },

            IconName {
                name: "FaPersonSkating".to_string(),
                icon: rsx!(Icon { icon: FaPersonSkating }),
            },

            IconName {
                name: "FaPersonSkiingNordic".to_string(),
                icon: rsx!(Icon { icon: FaPersonSkiingNordic }),
            },

            IconName {
                name: "FaPersonSkiing".to_string(),
                icon: rsx!(Icon { icon: FaPersonSkiing }),
            },

            IconName {
                name: "FaPersonSnowboarding".to_string(),
                icon: rsx!(Icon { icon: FaPersonSnowboarding }),
            },

            IconName {
                name: "FaPersonSwimming".to_string(),
                icon: rsx!(Icon { icon: FaPersonSwimming }),
            },

            IconName {
                name: "FaPersonThroughWindow".to_string(),
                icon: rsx!(Icon { icon: FaPersonThroughWindow }),
            },

            IconName {
                name: "FaPersonWalkingArrowLoopLeft".to_string(),
                icon: rsx!(Icon { icon: FaPersonWalkingArrowLoopLeft }),
            },

            IconName {
                name: "FaPersonWalkingArrowRight".to_string(),
                icon: rsx!(Icon { icon: FaPersonWalkingArrowRight }),
            },

            IconName {
                name: "FaPersonWalkingDashedLineArrowRight".to_string(),
                icon: rsx!(Icon { icon: FaPersonWalkingDashedLineArrowRight }),
            },

            IconName {
                name: "FaPersonWalkingLuggage".to_string(),
                icon: rsx!(Icon { icon: FaPersonWalkingLuggage }),
            },

            IconName {
                name: "FaPersonWalkingWithCane".to_string(),
                icon: rsx!(Icon { icon: FaPersonWalkingWithCane }),
            },

            IconName {
                name: "FaPersonWalking".to_string(),
                icon: rsx!(Icon { icon: FaPersonWalking }),
            },

            IconName {
                name: "FaPerson".to_string(),
                icon: rsx!(Icon { icon: FaPerson }),
            },

            IconName {
                name: "FaPesetaSign".to_string(),
                icon: rsx!(Icon { icon: FaPesetaSign }),
            },

            IconName {
                name: "FaPesoSign".to_string(),
                icon: rsx!(Icon { icon: FaPesoSign }),
            },

            IconName {
                name: "FaPhoneFlip".to_string(),
                icon: rsx!(Icon { icon: FaPhoneFlip }),
            },

            IconName {
                name: "FaPhoneSlash".to_string(),
                icon: rsx!(Icon { icon: FaPhoneSlash }),
            },

            IconName {
                name: "FaPhoneVolume".to_string(),
                icon: rsx!(Icon { icon: FaPhoneVolume }),
            },

            IconName {
                name: "FaPhone".to_string(),
                icon: rsx!(Icon { icon: FaPhone }),
            },

            IconName {
                name: "FaPhotoFilm".to_string(),
                icon: rsx!(Icon { icon: FaPhotoFilm }),
            },

            IconName {
                name: "FaPiggyBank".to_string(),
                icon: rsx!(Icon { icon: FaPiggyBank }),
            },

            IconName {
                name: "FaPills".to_string(),
                icon: rsx!(Icon { icon: FaPills }),
            },

            IconName {
                name: "FaPizzaSlice".to_string(),
                icon: rsx!(Icon { icon: FaPizzaSlice }),
            },

            IconName {
                name: "FaPlaceOfWorship".to_string(),
                icon: rsx!(Icon { icon: FaPlaceOfWorship }),
            },

            IconName {
                name: "FaPlaneArrival".to_string(),
                icon: rsx!(Icon { icon: FaPlaneArrival }),
            },

            IconName {
                name: "FaPlaneCircleCheck".to_string(),
                icon: rsx!(Icon { icon: FaPlaneCircleCheck }),
            },

            IconName {
                name: "FaPlaneCircleExclamation".to_string(),
                icon: rsx!(Icon { icon: FaPlaneCircleExclamation }),
            },

            IconName {
                name: "FaPlaneCircleXmark".to_string(),
                icon: rsx!(Icon { icon: FaPlaneCircleXmark }),
            },

            IconName {
                name: "FaPlaneDeparture".to_string(),
                icon: rsx!(Icon { icon: FaPlaneDeparture }),
            },

            IconName {
                name: "FaPlaneLock".to_string(),
                icon: rsx!(Icon { icon: FaPlaneLock }),
            },

            IconName {
                name: "FaPlaneSlash".to_string(),
                icon: rsx!(Icon { icon: FaPlaneSlash }),
            },

            IconName {
                name: "FaPlaneUp".to_string(),
                icon: rsx!(Icon { icon: FaPlaneUp }),
            },

            IconName {
                name: "FaPlane".to_string(),
                icon: rsx!(Icon { icon: FaPlane }),
            },

            IconName {
                name: "FaPlantWilt".to_string(),
                icon: rsx!(Icon { icon: FaPlantWilt }),
            },

            IconName {
                name: "FaPlateWheat".to_string(),
                icon: rsx!(Icon { icon: FaPlateWheat }),
            },

            IconName {
                name: "FaPlay".to_string(),
                icon: rsx!(Icon { icon: FaPlay }),
            },

            IconName {
                name: "FaPlugCircleBolt".to_string(),
                icon: rsx!(Icon { icon: FaPlugCircleBolt }),
            },

            IconName {
                name: "FaPlugCircleCheck".to_string(),
                icon: rsx!(Icon { icon: FaPlugCircleCheck }),
            },

            IconName {
                name: "FaPlugCircleExclamation".to_string(),
                icon: rsx!(Icon { icon: FaPlugCircleExclamation }),
            },

            IconName {
                name: "FaPlugCircleMinus".to_string(),
                icon: rsx!(Icon { icon: FaPlugCircleMinus }),
            },

            IconName {
                name: "FaPlugCirclePlus".to_string(),
                icon: rsx!(Icon { icon: FaPlugCirclePlus }),
            },

            IconName {
                name: "FaPlugCircleXmark".to_string(),
                icon: rsx!(Icon { icon: FaPlugCircleXmark }),
            },

            IconName {
                name: "FaPlug".to_string(),
                icon: rsx!(Icon { icon: FaPlug }),
            },

            IconName {
                name: "FaPlusMinus".to_string(),
                icon: rsx!(Icon { icon: FaPlusMinus }),
            },

            IconName {
                name: "FaPlus".to_string(),
                icon: rsx!(Icon { icon: FaPlus }),
            },

            IconName {
                name: "FaPodcast".to_string(),
                icon: rsx!(Icon { icon: FaPodcast }),
            },

            IconName {
                name: "FaPooStorm".to_string(),
                icon: rsx!(Icon { icon: FaPooStorm }),
            },

            IconName {
                name: "FaPoo".to_string(),
                icon: rsx!(Icon { icon: FaPoo }),
            },

            IconName {
                name: "FaPoop".to_string(),
                icon: rsx!(Icon { icon: FaPoop }),
            },

            IconName {
                name: "FaPowerOff".to_string(),
                icon: rsx!(Icon { icon: FaPowerOff }),
            },

            IconName {
                name: "FaPrescriptionBottleMedical".to_string(),
                icon: rsx!(Icon { icon: FaPrescriptionBottleMedical }),
            },

            IconName {
                name: "FaPrescriptionBottle".to_string(),
                icon: rsx!(Icon { icon: FaPrescriptionBottle }),
            },

            IconName {
                name: "FaPrescription".to_string(),
                icon: rsx!(Icon { icon: FaPrescription }),
            },

            IconName {
                name: "FaPrint".to_string(),
                icon: rsx!(Icon { icon: FaPrint }),
            },

            IconName {
                name: "FaPumpMedical".to_string(),
                icon: rsx!(Icon { icon: FaPumpMedical }),
            },

            IconName {
                name: "FaPumpSoap".to_string(),
                icon: rsx!(Icon { icon: FaPumpSoap }),
            },

            IconName {
                name: "FaPuzzlePiece".to_string(),
                icon: rsx!(Icon { icon: FaPuzzlePiece }),
            },

            IconName {
                name: "FaQ".to_string(),
                icon: rsx!(Icon { icon: FaQ }),
            },

            IconName {
                name: "FaQrcode".to_string(),
                icon: rsx!(Icon { icon: FaQrcode }),
            },

            IconName {
                name: "FaQuestion".to_string(),
                icon: rsx!(Icon { icon: FaQuestion }),
            },

            IconName {
                name: "FaQuoteLeft".to_string(),
                icon: rsx!(Icon { icon: FaQuoteLeft }),
            },

            IconName {
                name: "FaQuoteRight".to_string(),
                icon: rsx!(Icon { icon: FaQuoteRight }),
            },

            IconName {
                name: "FaR".to_string(),
                icon: rsx!(Icon { icon: FaR }),
            },

            IconName {
                name: "FaRadiation".to_string(),
                icon: rsx!(Icon { icon: FaRadiation }),
            },

            IconName {
                name: "FaRadio".to_string(),
                icon: rsx!(Icon { icon: FaRadio }),
            },

            IconName {
                name: "FaRainbow".to_string(),
                icon: rsx!(Icon { icon: FaRainbow }),
            },

            IconName {
                name: "FaRankingStar".to_string(),
                icon: rsx!(Icon { icon: FaRankingStar }),
            },

            IconName {
                name: "FaReceipt".to_string(),
                icon: rsx!(Icon { icon: FaReceipt }),
            },

            IconName {
                name: "FaRecordVinyl".to_string(),
                icon: rsx!(Icon { icon: FaRecordVinyl }),
            },

            IconName {
                name: "FaRectangleAd".to_string(),
                icon: rsx!(Icon { icon: FaRectangleAd }),
            },

            IconName {
                name: "FaRectangleList".to_string(),
                icon: rsx!(Icon { icon: FaRectangleList }),
            },

            IconName {
                name: "FaRectangleXmark".to_string(),
                icon: rsx!(Icon { icon: FaRectangleXmark }),
            },

            IconName {
                name: "FaRecycle".to_string(),
                icon: rsx!(Icon { icon: FaRecycle }),
            },

            IconName {
                name: "FaRegistered".to_string(),
                icon: rsx!(Icon { icon: FaRegistered }),
            },

            IconName {
                name: "FaRepeat".to_string(),
                icon: rsx!(Icon { icon: FaRepeat }),
            },

            IconName {
                name: "FaReplyAll".to_string(),
                icon: rsx!(Icon { icon: FaReplyAll }),
            },

            IconName {
                name: "FaReply".to_string(),
                icon: rsx!(Icon { icon: FaReply }),
            },

            IconName {
                name: "FaRepublican".to_string(),
                icon: rsx!(Icon { icon: FaRepublican }),
            },

            IconName {
                name: "FaRestroom".to_string(),
                icon: rsx!(Icon { icon: FaRestroom }),
            },

            IconName {
                name: "FaRetweet".to_string(),
                icon: rsx!(Icon { icon: FaRetweet }),
            },

            IconName {
                name: "FaRibbon".to_string(),
                icon: rsx!(Icon { icon: FaRibbon }),
            },

            IconName {
                name: "FaRightFromBracket".to_string(),
                icon: rsx!(Icon { icon: FaRightFromBracket }),
            },

            IconName {
                name: "FaRightLeft".to_string(),
                icon: rsx!(Icon { icon: FaRightLeft }),
            },

            IconName {
                name: "FaRightLong".to_string(),
                icon: rsx!(Icon { icon: FaRightLong }),
            },

            IconName {
                name: "FaRightToBracket".to_string(),
                icon: rsx!(Icon { icon: FaRightToBracket }),
            },

            IconName {
                name: "FaRing".to_string(),
                icon: rsx!(Icon { icon: FaRing }),
            },

            IconName {
                name: "FaRoadBarrier".to_string(),
                icon: rsx!(Icon { icon: FaRoadBarrier }),
            },

            IconName {
                name: "FaRoadBridge".to_string(),
                icon: rsx!(Icon { icon: FaRoadBridge }),
            },

            IconName {
                name: "FaRoadCircleCheck".to_string(),
                icon: rsx!(Icon { icon: FaRoadCircleCheck }),
            },

            IconName {
                name: "FaRoadCircleExclamation".to_string(),
                icon: rsx!(Icon { icon: FaRoadCircleExclamation }),
            },

            IconName {
                name: "FaRoadCircleXmark".to_string(),
                icon: rsx!(Icon { icon: FaRoadCircleXmark }),
            },

            IconName {
                name: "FaRoadLock".to_string(),
                icon: rsx!(Icon { icon: FaRoadLock }),
            },

            IconName {
                name: "FaRoadSpikes".to_string(),
                icon: rsx!(Icon { icon: FaRoadSpikes }),
            },

            IconName {
                name: "FaRoad".to_string(),
                icon: rsx!(Icon { icon: FaRoad }),
            },

            IconName {
                name: "FaRobot".to_string(),
                icon: rsx!(Icon { icon: FaRobot }),
            },

            IconName {
                name: "FaRocket".to_string(),
                icon: rsx!(Icon { icon: FaRocket }),
            },

            IconName {
                name: "FaRotateLeft".to_string(),
                icon: rsx!(Icon { icon: FaRotateLeft }),
            },

            IconName {
                name: "FaRotateRight".to_string(),
                icon: rsx!(Icon { icon: FaRotateRight }),
            },

            IconName {
                name: "FaRotate".to_string(),
                icon: rsx!(Icon { icon: FaRotate }),
            },

            IconName {
                name: "FaRoute".to_string(),
                icon: rsx!(Icon { icon: FaRoute }),
            },

            IconName {
                name: "FaRss".to_string(),
                icon: rsx!(Icon { icon: FaRss }),
            },

            IconName {
                name: "FaRubleSign".to_string(),
                icon: rsx!(Icon { icon: FaRubleSign }),
            },

            IconName {
                name: "FaRug".to_string(),
                icon: rsx!(Icon { icon: FaRug }),
            },

            IconName {
                name: "FaRulerCombined".to_string(),
                icon: rsx!(Icon { icon: FaRulerCombined }),
            },

            IconName {
                name: "FaRulerHorizontal".to_string(),
                icon: rsx!(Icon { icon: FaRulerHorizontal }),
            },

            IconName {
                name: "FaRulerVertical".to_string(),
                icon: rsx!(Icon { icon: FaRulerVertical }),
            },

            IconName {
                name: "FaRuler".to_string(),
                icon: rsx!(Icon { icon: FaRuler }),
            },

            IconName {
                name: "FaRupeeSign".to_string(),
                icon: rsx!(Icon { icon: FaRupeeSign }),
            },

            IconName {
                name: "FaRupiahSign".to_string(),
                icon: rsx!(Icon { icon: FaRupiahSign }),
            },

            IconName {
                name: "FaS".to_string(),
                icon: rsx!(Icon { icon: FaS }),
            },

            IconName {
                name: "FaSackDollar".to_string(),
                icon: rsx!(Icon { icon: FaSackDollar }),
            },

            IconName {
                name: "FaSackXmark".to_string(),
                icon: rsx!(Icon { icon: FaSackXmark }),
            },

            IconName {
                name: "FaSailboat".to_string(),
                icon: rsx!(Icon { icon: FaSailboat }),
            },

            IconName {
                name: "FaSatelliteDish".to_string(),
                icon: rsx!(Icon { icon: FaSatelliteDish }),
            },

            IconName {
                name: "FaSatellite".to_string(),
                icon: rsx!(Icon { icon: FaSatellite }),
            },

            IconName {
                name: "FaScaleBalanced".to_string(),
                icon: rsx!(Icon { icon: FaScaleBalanced }),
            },

            IconName {
                name: "FaScaleUnbalancedFlip".to_string(),
                icon: rsx!(Icon { icon: FaScaleUnbalancedFlip }),
            },

            IconName {
                name: "FaScaleUnbalanced".to_string(),
                icon: rsx!(Icon { icon: FaScaleUnbalanced }),
            },

            IconName {
                name: "FaSchoolCircleCheck".to_string(),
                icon: rsx!(Icon { icon: FaSchoolCircleCheck }),
            },

            IconName {
                name: "FaSchoolCircleExclamation".to_string(),
                icon: rsx!(Icon { icon: FaSchoolCircleExclamation }),
            },

            IconName {
                name: "FaSchoolCircleXmark".to_string(),
                icon: rsx!(Icon { icon: FaSchoolCircleXmark }),
            },

            IconName {
                name: "FaSchoolFlag".to_string(),
                icon: rsx!(Icon { icon: FaSchoolFlag }),
            },

            IconName {
                name: "FaSchoolLock".to_string(),
                icon: rsx!(Icon { icon: FaSchoolLock }),
            },

            IconName {
                name: "FaSchool".to_string(),
                icon: rsx!(Icon { icon: FaSchool }),
            },

            IconName {
                name: "FaScissors".to_string(),
                icon: rsx!(Icon { icon: FaScissors }),
            },

            IconName {
                name: "FaScrewdriverWrench".to_string(),
                icon: rsx!(Icon { icon: FaScrewdriverWrench }),
            },

            IconName {
                name: "FaScrewdriver".to_string(),
                icon: rsx!(Icon { icon: FaScrewdriver }),
            },

            IconName {
                name: "FaScrollTorah".to_string(),
                icon: rsx!(Icon { icon: FaScrollTorah }),
            },

            IconName {
                name: "FaScroll".to_string(),
                icon: rsx!(Icon { icon: FaScroll }),
            },

            IconName {
                name: "FaSdCard".to_string(),
                icon: rsx!(Icon { icon: FaSdCard }),
            },

            IconName {
                name: "FaSection".to_string(),
                icon: rsx!(Icon { icon: FaSection }),
            },

            IconName {
                name: "FaSeedling".to_string(),
                icon: rsx!(Icon { icon: FaSeedling }),
            },

            IconName {
                name: "FaServer".to_string(),
                icon: rsx!(Icon { icon: FaServer }),
            },

            IconName {
                name: "FaShapes".to_string(),
                icon: rsx!(Icon { icon: FaShapes }),
            },

            IconName {
                name: "FaShareFromSquare".to_string(),
                icon: rsx!(Icon { icon: FaShareFromSquare }),
            },

            IconName {
                name: "FaShareNodes".to_string(),
                icon: rsx!(Icon { icon: FaShareNodes }),
            },

            IconName {
                name: "FaShare".to_string(),
                icon: rsx!(Icon { icon: FaShare }),
            },

            IconName {
                name: "FaSheetPlastic".to_string(),
                icon: rsx!(Icon { icon: FaSheetPlastic }),
            },

            IconName {
                name: "FaShekelSign".to_string(),
                icon: rsx!(Icon { icon: FaShekelSign }),
            },

            IconName {
                name: "FaShieldBlank".to_string(),
                icon: rsx!(Icon { icon: FaShieldBlank }),
            },

            IconName {
                name: "FaShieldCat".to_string(),
                icon: rsx!(Icon { icon: FaShieldCat }),
            },

            IconName {
                name: "FaShieldDog".to_string(),
                icon: rsx!(Icon { icon: FaShieldDog }),
            },

            IconName {
                name: "FaShieldHalved".to_string(),
                icon: rsx!(Icon { icon: FaShieldHalved }),
            },

            IconName {
                name: "FaShieldHeart".to_string(),
                icon: rsx!(Icon { icon: FaShieldHeart }),
            },

            IconName {
                name: "FaShieldVirus".to_string(),
                icon: rsx!(Icon { icon: FaShieldVirus }),
            },

            IconName {
                name: "FaShield".to_string(),
                icon: rsx!(Icon { icon: FaShield }),
            },

            IconName {
                name: "FaShip".to_string(),
                icon: rsx!(Icon { icon: FaShip }),
            },

            IconName {
                name: "FaShirt".to_string(),
                icon: rsx!(Icon { icon: FaShirt }),
            },

            IconName {
                name: "FaShoePrints".to_string(),
                icon: rsx!(Icon { icon: FaShoePrints }),
            },

            IconName {
                name: "FaShopLock".to_string(),
                icon: rsx!(Icon { icon: FaShopLock }),
            },

            IconName {
                name: "FaShopSlash".to_string(),
                icon: rsx!(Icon { icon: FaShopSlash }),
            },

            IconName {
                name: "FaShop".to_string(),
                icon: rsx!(Icon { icon: FaShop }),
            },

            IconName {
                name: "FaShower".to_string(),
                icon: rsx!(Icon { icon: FaShower }),
            },

            IconName {
                name: "FaShrimp".to_string(),
                icon: rsx!(Icon { icon: FaShrimp }),
            },

            IconName {
                name: "FaShuffle".to_string(),
                icon: rsx!(Icon { icon: FaShuffle }),
            },

            IconName {
                name: "FaShuttleSpace".to_string(),
                icon: rsx!(Icon { icon: FaShuttleSpace }),
            },

            IconName {
                name: "FaSignHanging".to_string(),
                icon: rsx!(Icon { icon: FaSignHanging }),
            },

            IconName {
                name: "FaSignal".to_string(),
                icon: rsx!(Icon { icon: FaSignal }),
            },

            IconName {
                name: "FaSignature".to_string(),
                icon: rsx!(Icon { icon: FaSignature }),
            },

            IconName {
                name: "FaSignsPost".to_string(),
                icon: rsx!(Icon { icon: FaSignsPost }),
            },

            IconName {
                name: "FaSimCard".to_string(),
                icon: rsx!(Icon { icon: FaSimCard }),
            },

            IconName {
                name: "FaSink".to_string(),
                icon: rsx!(Icon { icon: FaSink }),
            },

            IconName {
                name: "FaSitemap".to_string(),
                icon: rsx!(Icon { icon: FaSitemap }),
            },

            IconName {
                name: "FaSkullCrossbones".to_string(),
                icon: rsx!(Icon { icon: FaSkullCrossbones }),
            },

            IconName {
                name: "FaSkull".to_string(),
                icon: rsx!(Icon { icon: FaSkull }),
            },

            IconName {
                name: "FaSlash".to_string(),
                icon: rsx!(Icon { icon: FaSlash }),
            },

            IconName {
                name: "FaSleigh".to_string(),
                icon: rsx!(Icon { icon: FaSleigh }),
            },

            IconName {
                name: "FaSliders".to_string(),
                icon: rsx!(Icon { icon: FaSliders }),
            },

            IconName {
                name: "FaSmog".to_string(),
                icon: rsx!(Icon { icon: FaSmog }),
            },

            IconName {
                name: "FaSmoking".to_string(),
                icon: rsx!(Icon { icon: FaSmoking }),
            },

            IconName {
                name: "FaSnowflake".to_string(),
                icon: rsx!(Icon { icon: FaSnowflake }),
            },

            IconName {
                name: "FaSnowman".to_string(),
                icon: rsx!(Icon { icon: FaSnowman }),
            },

            IconName {
                name: "FaSnowplow".to_string(),
                icon: rsx!(Icon { icon: FaSnowplow }),
            },

            IconName {
                name: "FaSoap".to_string(),
                icon: rsx!(Icon { icon: FaSoap }),
            },

            IconName {
                name: "FaSocks".to_string(),
                icon: rsx!(Icon { icon: FaSocks }),
            },

            IconName {
                name: "FaSolarPanel".to_string(),
                icon: rsx!(Icon { icon: FaSolarPanel }),
            },

            IconName {
                name: "FaSortDown".to_string(),
                icon: rsx!(Icon { icon: FaSortDown }),
            },

            IconName {
                name: "FaSortUp".to_string(),
                icon: rsx!(Icon { icon: FaSortUp }),
            },

            IconName {
                name: "FaSort".to_string(),
                icon: rsx!(Icon { icon: FaSort }),
            },

            IconName {
                name: "FaSpa".to_string(),
                icon: rsx!(Icon { icon: FaSpa }),
            },

            IconName {
                name: "FaSpaghettiMonsterFlying".to_string(),
                icon: rsx!(Icon { icon: FaSpaghettiMonsterFlying }),
            },

            IconName {
                name: "FaSpellCheck".to_string(),
                icon: rsx!(Icon { icon: FaSpellCheck }),
            },

            IconName {
                name: "FaSpider".to_string(),
                icon: rsx!(Icon { icon: FaSpider }),
            },

            IconName {
                name: "FaSpinner".to_string(),
                icon: rsx!(Icon { icon: FaSpinner }),
            },

            IconName {
                name: "FaSplotch".to_string(),
                icon: rsx!(Icon { icon: FaSplotch }),
            },

            IconName {
                name: "FaSpoon".to_string(),
                icon: rsx!(Icon { icon: FaSpoon }),
            },

            IconName {
                name: "FaSprayCanSparkles".to_string(),
                icon: rsx!(Icon { icon: FaSprayCanSparkles }),
            },

            IconName {
                name: "FaSprayCan".to_string(),
                icon: rsx!(Icon { icon: FaSprayCan }),
            },

            IconName {
                name: "FaSquareArrowUpRight".to_string(),
                icon: rsx!(Icon { icon: FaSquareArrowUpRight }),
            },

            IconName {
                name: "FaSquareCaretDown".to_string(),
                icon: rsx!(Icon { icon: FaSquareCaretDown }),
            },

            IconName {
                name: "FaSquareCaretLeft".to_string(),
                icon: rsx!(Icon { icon: FaSquareCaretLeft }),
            },

            IconName {
                name: "FaSquareCaretRight".to_string(),
                icon: rsx!(Icon { icon: FaSquareCaretRight }),
            },

            IconName {
                name: "FaSquareCaretUp".to_string(),
                icon: rsx!(Icon { icon: FaSquareCaretUp }),
            },

            IconName {
                name: "FaSquareCheck".to_string(),
                icon: rsx!(Icon { icon: FaSquareCheck }),
            },

            IconName {
                name: "FaSquareEnvelope".to_string(),
                icon: rsx!(Icon { icon: FaSquareEnvelope }),
            },

            IconName {
                name: "FaSquareFull".to_string(),
                icon: rsx!(Icon { icon: FaSquareFull }),
            },

            IconName {
                name: "FaSquareH".to_string(),
                icon: rsx!(Icon { icon: FaSquareH }),
            },

            IconName {
                name: "FaSquareMinus".to_string(),
                icon: rsx!(Icon { icon: FaSquareMinus }),
            },

            IconName {
                name: "FaSquareNfi".to_string(),
                icon: rsx!(Icon { icon: FaSquareNfi }),
            },

            IconName {
                name: "FaSquareParking".to_string(),
                icon: rsx!(Icon { icon: FaSquareParking }),
            },

            IconName {
                name: "FaSquarePen".to_string(),
                icon: rsx!(Icon { icon: FaSquarePen }),
            },

            IconName {
                name: "FaSquarePersonConfined".to_string(),
                icon: rsx!(Icon { icon: FaSquarePersonConfined }),
            },

            IconName {
                name: "FaSquarePhoneFlip".to_string(),
                icon: rsx!(Icon { icon: FaSquarePhoneFlip }),
            },

            IconName {
                name: "FaSquarePhone".to_string(),
                icon: rsx!(Icon { icon: FaSquarePhone }),
            },

            IconName {
                name: "FaSquarePlus".to_string(),
                icon: rsx!(Icon { icon: FaSquarePlus }),
            },

            IconName {
                name: "FaSquarePollHorizontal".to_string(),
                icon: rsx!(Icon { icon: FaSquarePollHorizontal }),
            },

            IconName {
                name: "FaSquarePollVertical".to_string(),
                icon: rsx!(Icon { icon: FaSquarePollVertical }),
            },

            IconName {
                name: "FaSquareRootVariable".to_string(),
                icon: rsx!(Icon { icon: FaSquareRootVariable }),
            },

            IconName {
                name: "FaSquareRss".to_string(),
                icon: rsx!(Icon { icon: FaSquareRss }),
            },

            IconName {
                name: "FaSquareShareNodes".to_string(),
                icon: rsx!(Icon { icon: FaSquareShareNodes }),
            },

            IconName {
                name: "FaSquareUpRight".to_string(),
                icon: rsx!(Icon { icon: FaSquareUpRight }),
            },

            IconName {
                name: "FaSquareVirus".to_string(),
                icon: rsx!(Icon { icon: FaSquareVirus }),
            },

            IconName {
                name: "FaSquareXmark".to_string(),
                icon: rsx!(Icon { icon: FaSquareXmark }),
            },

            IconName {
                name: "FaSquare".to_string(),
                icon: rsx!(Icon { icon: FaSquare }),
            },

            IconName {
                name: "FaStaffAesculapius".to_string(),
                icon: rsx!(Icon { icon: FaStaffAesculapius }),
            },

            IconName {
                name: "FaStairs".to_string(),
                icon: rsx!(Icon { icon: FaStairs }),
            },

            IconName {
                name: "FaStamp".to_string(),
                icon: rsx!(Icon { icon: FaStamp }),
            },

            IconName {
                name: "FaStarAndCrescent".to_string(),
                icon: rsx!(Icon { icon: FaStarAndCrescent }),
            },

            IconName {
                name: "FaStarHalfStroke".to_string(),
                icon: rsx!(Icon { icon: FaStarHalfStroke }),
            },

            IconName {
                name: "FaStarHalf".to_string(),
                icon: rsx!(Icon { icon: FaStarHalf }),
            },

            IconName {
                name: "FaStarOfDavid".to_string(),
                icon: rsx!(Icon { icon: FaStarOfDavid }),
            },

            IconName {
                name: "FaStarOfLife".to_string(),
                icon: rsx!(Icon { icon: FaStarOfLife }),
            },

            IconName {
                name: "FaStar".to_string(),
                icon: rsx!(Icon { icon: FaStar }),
            },

            IconName {
                name: "FaSterlingSign".to_string(),
                icon: rsx!(Icon { icon: FaSterlingSign }),
            },

            IconName {
                name: "FaStethoscope".to_string(),
                icon: rsx!(Icon { icon: FaStethoscope }),
            },

            IconName {
                name: "FaStop".to_string(),
                icon: rsx!(Icon { icon: FaStop }),
            },

            IconName {
                name: "FaStopwatch20".to_string(),
                icon: rsx!(Icon { icon: FaStopwatch20 }),
            },

            IconName {
                name: "FaStopwatch".to_string(),
                icon: rsx!(Icon { icon: FaStopwatch }),
            },

            IconName {
                name: "FaStoreSlash".to_string(),
                icon: rsx!(Icon { icon: FaStoreSlash }),
            },

            IconName {
                name: "FaStore".to_string(),
                icon: rsx!(Icon { icon: FaStore }),
            },

            IconName {
                name: "FaStreetView".to_string(),
                icon: rsx!(Icon { icon: FaStreetView }),
            },

            IconName {
                name: "FaStrikethrough".to_string(),
                icon: rsx!(Icon { icon: FaStrikethrough }),
            },

            IconName {
                name: "FaStroopwafel".to_string(),
                icon: rsx!(Icon { icon: FaStroopwafel }),
            },

            IconName {
                name: "FaSubscript".to_string(),
                icon: rsx!(Icon { icon: FaSubscript }),
            },

            IconName {
                name: "FaSuitcaseMedical".to_string(),
                icon: rsx!(Icon { icon: FaSuitcaseMedical }),
            },

            IconName {
                name: "FaSuitcaseRolling".to_string(),
                icon: rsx!(Icon { icon: FaSuitcaseRolling }),
            },

            IconName {
                name: "FaSuitcase".to_string(),
                icon: rsx!(Icon { icon: FaSuitcase }),
            },

            IconName {
                name: "FaSunPlantWilt".to_string(),
                icon: rsx!(Icon { icon: FaSunPlantWilt }),
            },

            IconName {
                name: "FaSun".to_string(),
                icon: rsx!(Icon { icon: FaSun }),
            },

            IconName {
                name: "FaSuperscript".to_string(),
                icon: rsx!(Icon { icon: FaSuperscript }),
            },

            IconName {
                name: "FaSwatchbook".to_string(),
                icon: rsx!(Icon { icon: FaSwatchbook }),
            },

            IconName {
                name: "FaSynagogue".to_string(),
                icon: rsx!(Icon { icon: FaSynagogue }),
            },

            IconName {
                name: "FaSyringe".to_string(),
                icon: rsx!(Icon { icon: FaSyringe }),
            },

            IconName {
                name: "FaT".to_string(),
                icon: rsx!(Icon { icon: FaT }),
            },

            IconName {
                name: "FaTableCellsLarge".to_string(),
                icon: rsx!(Icon { icon: FaTableCellsLarge }),
            },

            IconName {
                name: "FaTableCells".to_string(),
                icon: rsx!(Icon { icon: FaTableCells }),
            },

            IconName {
                name: "FaTableColumns".to_string(),
                icon: rsx!(Icon { icon: FaTableColumns }),
            },

            IconName {
                name: "FaTableList".to_string(),
                icon: rsx!(Icon { icon: FaTableList }),
            },

            IconName {
                name: "FaTableTennisPaddleBall".to_string(),
                icon: rsx!(Icon { icon: FaTableTennisPaddleBall }),
            },

            IconName {
                name: "FaTable".to_string(),
                icon: rsx!(Icon { icon: FaTable }),
            },

            IconName {
                name: "FaTabletButton".to_string(),
                icon: rsx!(Icon { icon: FaTabletButton }),
            },

            IconName {
                name: "FaTabletScreenButton".to_string(),
                icon: rsx!(Icon { icon: FaTabletScreenButton }),
            },

            IconName {
                name: "FaTablet".to_string(),
                icon: rsx!(Icon { icon: FaTablet }),
            },

            IconName {
                name: "FaTablets".to_string(),
                icon: rsx!(Icon { icon: FaTablets }),
            },

            IconName {
                name: "FaTachographDigital".to_string(),
                icon: rsx!(Icon { icon: FaTachographDigital }),
            },

            IconName {
                name: "FaTag".to_string(),
                icon: rsx!(Icon { icon: FaTag }),
            },

            IconName {
                name: "FaTags".to_string(),
                icon: rsx!(Icon { icon: FaTags }),
            },

            IconName {
                name: "FaTape".to_string(),
                icon: rsx!(Icon { icon: FaTape }),
            },

            IconName {
                name: "FaTarpDroplet".to_string(),
                icon: rsx!(Icon { icon: FaTarpDroplet }),
            },

            IconName {
                name: "FaTarp".to_string(),
                icon: rsx!(Icon { icon: FaTarp }),
            },

            IconName {
                name: "FaTaxi".to_string(),
                icon: rsx!(Icon { icon: FaTaxi }),
            },

            IconName {
                name: "FaTeethOpen".to_string(),
                icon: rsx!(Icon { icon: FaTeethOpen }),
            },

            IconName {
                name: "FaTeeth".to_string(),
                icon: rsx!(Icon { icon: FaTeeth }),
            },

            IconName {
                name: "FaTemperatureArrowDown".to_string(),
                icon: rsx!(Icon { icon: FaTemperatureArrowDown }),
            },

            IconName {
                name: "FaTemperatureArrowUp".to_string(),
                icon: rsx!(Icon { icon: FaTemperatureArrowUp }),
            },

            IconName {
                name: "FaTemperatureEmpty".to_string(),
                icon: rsx!(Icon { icon: FaTemperatureEmpty }),
            },

            IconName {
                name: "FaTemperatureFull".to_string(),
                icon: rsx!(Icon { icon: FaTemperatureFull }),
            },

            IconName {
                name: "FaTemperatureHalf".to_string(),
                icon: rsx!(Icon { icon: FaTemperatureHalf }),
            },

            IconName {
                name: "FaTemperatureHigh".to_string(),
                icon: rsx!(Icon { icon: FaTemperatureHigh }),
            },

            IconName {
                name: "FaTemperatureLow".to_string(),
                icon: rsx!(Icon { icon: FaTemperatureLow }),
            },

            IconName {
                name: "FaTemperatureQuarter".to_string(),
                icon: rsx!(Icon { icon: FaTemperatureQuarter }),
            },

            IconName {
                name: "FaTemperatureThreeQuarters".to_string(),
                icon: rsx!(Icon { icon: FaTemperatureThreeQuarters }),
            },

            IconName {
                name: "FaTengeSign".to_string(),
                icon: rsx!(Icon { icon: FaTengeSign }),
            },

            IconName {
                name: "FaTentArrowDownToLine".to_string(),
                icon: rsx!(Icon { icon: FaTentArrowDownToLine }),
            },

            IconName {
                name: "FaTentArrowLeftRight".to_string(),
                icon: rsx!(Icon { icon: FaTentArrowLeftRight }),
            },

            IconName {
                name: "FaTentArrowTurnLeft".to_string(),
                icon: rsx!(Icon { icon: FaTentArrowTurnLeft }),
            },

            IconName {
                name: "FaTentArrowsDown".to_string(),
                icon: rsx!(Icon { icon: FaTentArrowsDown }),
            },

            IconName {
                name: "FaTent".to_string(),
                icon: rsx!(Icon { icon: FaTent }),
            },

            IconName {
                name: "FaTents".to_string(),
                icon: rsx!(Icon { icon: FaTents }),
            },

            IconName {
                name: "FaTerminal".to_string(),
                icon: rsx!(Icon { icon: FaTerminal }),
            },

            IconName {
                name: "FaTextHeight".to_string(),
                icon: rsx!(Icon { icon: FaTextHeight }),
            },

            IconName {
                name: "FaTextSlash".to_string(),
                icon: rsx!(Icon { icon: FaTextSlash }),
            },

            IconName {
                name: "FaTextWidth".to_string(),
                icon: rsx!(Icon { icon: FaTextWidth }),
            },

            IconName {
                name: "FaThermometer".to_string(),
                icon: rsx!(Icon { icon: FaThermometer }),
            },

            IconName {
                name: "FaThumbsDown".to_string(),
                icon: rsx!(Icon { icon: FaThumbsDown }),
            },

            IconName {
                name: "FaThumbsUp".to_string(),
                icon: rsx!(Icon { icon: FaThumbsUp }),
            },

            IconName {
                name: "FaThumbtack".to_string(),
                icon: rsx!(Icon { icon: FaThumbtack }),
            },

            IconName {
                name: "FaTicketSimple".to_string(),
                icon: rsx!(Icon { icon: FaTicketSimple }),
            },

            IconName {
                name: "FaTicket".to_string(),
                icon: rsx!(Icon { icon: FaTicket }),
            },

            IconName {
                name: "FaTimeline".to_string(),
                icon: rsx!(Icon { icon: FaTimeline }),
            },

            IconName {
                name: "FaToggleOff".to_string(),
                icon: rsx!(Icon { icon: FaToggleOff }),
            },

            IconName {
                name: "FaToggleOn".to_string(),
                icon: rsx!(Icon { icon: FaToggleOn }),
            },

            IconName {
                name: "FaToiletPaperSlash".to_string(),
                icon: rsx!(Icon { icon: FaToiletPaperSlash }),
            },

            IconName {
                name: "FaToiletPaper".to_string(),
                icon: rsx!(Icon { icon: FaToiletPaper }),
            },

            IconName {
                name: "FaToiletPortable".to_string(),
                icon: rsx!(Icon { icon: FaToiletPortable }),
            },

            IconName {
                name: "FaToilet".to_string(),
                icon: rsx!(Icon { icon: FaToilet }),
            },

            IconName {
                name: "FaToiletsPortable".to_string(),
                icon: rsx!(Icon { icon: FaToiletsPortable }),
            },

            IconName {
                name: "FaToolbox".to_string(),
                icon: rsx!(Icon { icon: FaToolbox }),
            },

            IconName {
                name: "FaTooth".to_string(),
                icon: rsx!(Icon { icon: FaTooth }),
            },

            IconName {
                name: "FaToriiGate".to_string(),
                icon: rsx!(Icon { icon: FaToriiGate }),
            },

            IconName {
                name: "FaTornado".to_string(),
                icon: rsx!(Icon { icon: FaTornado }),
            },

            IconName {
                name: "FaTowerBroadcast".to_string(),
                icon: rsx!(Icon { icon: FaTowerBroadcast }),
            },

            IconName {
                name: "FaTowerCell".to_string(),
                icon: rsx!(Icon { icon: FaTowerCell }),
            },

            IconName {
                name: "FaTowerObservation".to_string(),
                icon: rsx!(Icon { icon: FaTowerObservation }),
            },

            IconName {
                name: "FaTractor".to_string(),
                icon: rsx!(Icon { icon: FaTractor }),
            },

            IconName {
                name: "FaTrademark".to_string(),
                icon: rsx!(Icon { icon: FaTrademark }),
            },

            IconName {
                name: "FaTrafficLight".to_string(),
                icon: rsx!(Icon { icon: FaTrafficLight }),
            },

            IconName {
                name: "FaTrailer".to_string(),
                icon: rsx!(Icon { icon: FaTrailer }),
            },

            IconName {
                name: "FaTrainSubway".to_string(),
                icon: rsx!(Icon { icon: FaTrainSubway }),
            },

            IconName {
                name: "FaTrainTram".to_string(),
                icon: rsx!(Icon { icon: FaTrainTram }),
            },

            IconName {
                name: "FaTrain".to_string(),
                icon: rsx!(Icon { icon: FaTrain }),
            },

            IconName {
                name: "FaTransgender".to_string(),
                icon: rsx!(Icon { icon: FaTransgender }),
            },

            IconName {
                name: "FaTrashArrowUp".to_string(),
                icon: rsx!(Icon { icon: FaTrashArrowUp }),
            },

            IconName {
                name: "FaTrashCanArrowUp".to_string(),
                icon: rsx!(Icon { icon: FaTrashCanArrowUp }),
            },

            IconName {
                name: "FaTrashCan".to_string(),
                icon: rsx!(Icon { icon: FaTrashCan }),
            },

            IconName {
                name: "FaTrash".to_string(),
                icon: rsx!(Icon { icon: FaTrash }),
            },

            IconName {
                name: "FaTreeCity".to_string(),
                icon: rsx!(Icon { icon: FaTreeCity }),
            },

            IconName {
                name: "FaTree".to_string(),
                icon: rsx!(Icon { icon: FaTree }),
            },

            IconName {
                name: "FaTriangleExclamation".to_string(),
                icon: rsx!(Icon { icon: FaTriangleExclamation }),
            },

            IconName {
                name: "FaTrophy".to_string(),
                icon: rsx!(Icon { icon: FaTrophy }),
            },

            IconName {
                name: "FaTrowelBricks".to_string(),
                icon: rsx!(Icon { icon: FaTrowelBricks }),
            },

            IconName {
                name: "FaTrowel".to_string(),
                icon: rsx!(Icon { icon: FaTrowel }),
            },

            IconName {
                name: "FaTruckArrowRight".to_string(),
                icon: rsx!(Icon { icon: FaTruckArrowRight }),
            },

            IconName {
                name: "FaTruckDroplet".to_string(),
                icon: rsx!(Icon { icon: FaTruckDroplet }),
            },

            IconName {
                name: "FaTruckFast".to_string(),
                icon: rsx!(Icon { icon: FaTruckFast }),
            },

            IconName {
                name: "FaTruckFieldUn".to_string(),
                icon: rsx!(Icon { icon: FaTruckFieldUn }),
            },

            IconName {
                name: "FaTruckField".to_string(),
                icon: rsx!(Icon { icon: FaTruckField }),
            },

            IconName {
                name: "FaTruckFront".to_string(),
                icon: rsx!(Icon { icon: FaTruckFront }),
            },

            IconName {
                name: "FaTruckMedical".to_string(),
                icon: rsx!(Icon { icon: FaTruckMedical }),
            },

            IconName {
                name: "FaTruckMonster".to_string(),
                icon: rsx!(Icon { icon: FaTruckMonster }),
            },

            IconName {
                name: "FaTruckMoving".to_string(),
                icon: rsx!(Icon { icon: FaTruckMoving }),
            },

            IconName {
                name: "FaTruckPickup".to_string(),
                icon: rsx!(Icon { icon: FaTruckPickup }),
            },

            IconName {
                name: "FaTruckPlane".to_string(),
                icon: rsx!(Icon { icon: FaTruckPlane }),
            },

            IconName {
                name: "FaTruckRampBox".to_string(),
                icon: rsx!(Icon { icon: FaTruckRampBox }),
            },

            IconName {
                name: "FaTruck".to_string(),
                icon: rsx!(Icon { icon: FaTruck }),
            },

            IconName {
                name: "FaTty".to_string(),
                icon: rsx!(Icon { icon: FaTty }),
            },

            IconName {
                name: "FaTurkishLiraSign".to_string(),
                icon: rsx!(Icon { icon: FaTurkishLiraSign }),
            },

            IconName {
                name: "FaTurnDown".to_string(),
                icon: rsx!(Icon { icon: FaTurnDown }),
            },

            IconName {
                name: "FaTurnUp".to_string(),
                icon: rsx!(Icon { icon: FaTurnUp }),
            },

            IconName {
                name: "FaTv".to_string(),
                icon: rsx!(Icon { icon: FaTv }),
            },

            IconName {
                name: "FaU".to_string(),
                icon: rsx!(Icon { icon: FaU }),
            },

            IconName {
                name: "FaUmbrellaBeach".to_string(),
                icon: rsx!(Icon { icon: FaUmbrellaBeach }),
            },

            IconName {
                name: "FaUmbrella".to_string(),
                icon: rsx!(Icon { icon: FaUmbrella }),
            },

            IconName {
                name: "FaUnderline".to_string(),
                icon: rsx!(Icon { icon: FaUnderline }),
            },

            IconName {
                name: "FaUniversalAccess".to_string(),
                icon: rsx!(Icon { icon: FaUniversalAccess }),
            },

            IconName {
                name: "FaUnlockKeyhole".to_string(),
                icon: rsx!(Icon { icon: FaUnlockKeyhole }),
            },

            IconName {
                name: "FaUnlock".to_string(),
                icon: rsx!(Icon { icon: FaUnlock }),
            },

            IconName {
                name: "FaUpDownLeftRight".to_string(),
                icon: rsx!(Icon { icon: FaUpDownLeftRight }),
            },

            IconName {
                name: "FaUpDown".to_string(),
                icon: rsx!(Icon { icon: FaUpDown }),
            },

            IconName {
                name: "FaUpLong".to_string(),
                icon: rsx!(Icon { icon: FaUpLong }),
            },

            IconName {
                name: "FaUpRightAndDownLeftFromCenter".to_string(),
                icon: rsx!(Icon { icon: FaUpRightAndDownLeftFromCenter }),
            },

            IconName {
                name: "FaUpRightFromSquare".to_string(),
                icon: rsx!(Icon { icon: FaUpRightFromSquare }),
            },

            IconName {
                name: "FaUpload".to_string(),
                icon: rsx!(Icon { icon: FaUpload }),
            },

            IconName {
                name: "FaUserAstronaut".to_string(),
                icon: rsx!(Icon { icon: FaUserAstronaut }),
            },

            IconName {
                name: "FaUserCheck".to_string(),
                icon: rsx!(Icon { icon: FaUserCheck }),
            },

            IconName {
                name: "FaUserClock".to_string(),
                icon: rsx!(Icon { icon: FaUserClock }),
            },

            IconName {
                name: "FaUserDoctor".to_string(),
                icon: rsx!(Icon { icon: FaUserDoctor }),
            },

            IconName {
                name: "FaUserGear".to_string(),
                icon: rsx!(Icon { icon: FaUserGear }),
            },

            IconName {
                name: "FaUserGraduate".to_string(),
                icon: rsx!(Icon { icon: FaUserGraduate }),
            },

            IconName {
                name: "FaUserGroup".to_string(),
                icon: rsx!(Icon { icon: FaUserGroup }),
            },

            IconName {
                name: "FaUserInjured".to_string(),
                icon: rsx!(Icon { icon: FaUserInjured }),
            },

            IconName {
                name: "FaUserLargeSlash".to_string(),
                icon: rsx!(Icon { icon: FaUserLargeSlash }),
            },

            IconName {
                name: "FaUserLarge".to_string(),
                icon: rsx!(Icon { icon: FaUserLarge }),
            },

            IconName {
                name: "FaUserLock".to_string(),
                icon: rsx!(Icon { icon: FaUserLock }),
            },

            IconName {
                name: "FaUserMinus".to_string(),
                icon: rsx!(Icon { icon: FaUserMinus }),
            },

            IconName {
                name: "FaUserNinja".to_string(),
                icon: rsx!(Icon { icon: FaUserNinja }),
            },

            IconName {
                name: "FaUserNurse".to_string(),
                icon: rsx!(Icon { icon: FaUserNurse }),
            },

            IconName {
                name: "FaUserPen".to_string(),
                icon: rsx!(Icon { icon: FaUserPen }),
            },

            IconName {
                name: "FaUserPlus".to_string(),
                icon: rsx!(Icon { icon: FaUserPlus }),
            },

            IconName {
                name: "FaUserSecret".to_string(),
                icon: rsx!(Icon { icon: FaUserSecret }),
            },

            IconName {
                name: "FaUserShield".to_string(),
                icon: rsx!(Icon { icon: FaUserShield }),
            },

            IconName {
                name: "FaUserSlash".to_string(),
                icon: rsx!(Icon { icon: FaUserSlash }),
            },

            IconName {
                name: "FaUserTag".to_string(),
                icon: rsx!(Icon { icon: FaUserTag }),
            },

            IconName {
                name: "FaUserTie".to_string(),
                icon: rsx!(Icon { icon: FaUserTie }),
            },

            IconName {
                name: "FaUserXmark".to_string(),
                icon: rsx!(Icon { icon: FaUserXmark }),
            },

            IconName {
                name: "FaUser".to_string(),
                icon: rsx!(Icon { icon: FaUser }),
            },

            IconName {
                name: "FaUsersBetweenLines".to_string(),
                icon: rsx!(Icon { icon: FaUsersBetweenLines }),
            },

            IconName {
                name: "FaUsersGear".to_string(),
                icon: rsx!(Icon { icon: FaUsersGear }),
            },

            IconName {
                name: "FaUsersLine".to_string(),
                icon: rsx!(Icon { icon: FaUsersLine }),
            },

            IconName {
                name: "FaUsersRays".to_string(),
                icon: rsx!(Icon { icon: FaUsersRays }),
            },

            IconName {
                name: "FaUsersRectangle".to_string(),
                icon: rsx!(Icon { icon: FaUsersRectangle }),
            },

            IconName {
                name: "FaUsersSlash".to_string(),
                icon: rsx!(Icon { icon: FaUsersSlash }),
            },

            IconName {
                name: "FaUsersViewfinder".to_string(),
                icon: rsx!(Icon { icon: FaUsersViewfinder }),
            },

            IconName {
                name: "FaUsers".to_string(),
                icon: rsx!(Icon { icon: FaUsers }),
            },

            IconName {
                name: "FaUtensils".to_string(),
                icon: rsx!(Icon { icon: FaUtensils }),
            },

            IconName {
                name: "FaV".to_string(),
                icon: rsx!(Icon { icon: FaV }),
            },

            IconName {
                name: "FaVanShuttle".to_string(),
                icon: rsx!(Icon { icon: FaVanShuttle }),
            },

            IconName {
                name: "FaVault".to_string(),
                icon: rsx!(Icon { icon: FaVault }),
            },

            IconName {
                name: "FaVectorSquare".to_string(),
                icon: rsx!(Icon { icon: FaVectorSquare }),
            },

            IconName {
                name: "FaVenusDouble".to_string(),
                icon: rsx!(Icon { icon: FaVenusDouble }),
            },

            IconName {
                name: "FaVenusMars".to_string(),
                icon: rsx!(Icon { icon: FaVenusMars }),
            },

            IconName {
                name: "FaVenus".to_string(),
                icon: rsx!(Icon { icon: FaVenus }),
            },

            IconName {
                name: "FaVestPatches".to_string(),
                icon: rsx!(Icon { icon: FaVestPatches }),
            },

            IconName {
                name: "FaVest".to_string(),
                icon: rsx!(Icon { icon: FaVest }),
            },

            IconName {
                name: "FaVialCircleCheck".to_string(),
                icon: rsx!(Icon { icon: FaVialCircleCheck }),
            },

            IconName {
                name: "FaVialVirus".to_string(),
                icon: rsx!(Icon { icon: FaVialVirus }),
            },

            IconName {
                name: "FaVial".to_string(),
                icon: rsx!(Icon { icon: FaVial }),
            },

            IconName {
                name: "FaVials".to_string(),
                icon: rsx!(Icon { icon: FaVials }),
            },

            IconName {
                name: "FaVideoSlash".to_string(),
                icon: rsx!(Icon { icon: FaVideoSlash }),
            },

            IconName {
                name: "FaVideo".to_string(),
                icon: rsx!(Icon { icon: FaVideo }),
            },

            IconName {
                name: "FaVihara".to_string(),
                icon: rsx!(Icon { icon: FaVihara }),
            },

            IconName {
                name: "FaVirusCovidSlash".to_string(),
                icon: rsx!(Icon { icon: FaVirusCovidSlash }),
            },

            IconName {
                name: "FaVirusCovid".to_string(),
                icon: rsx!(Icon { icon: FaVirusCovid }),
            },

            IconName {
                name: "FaVirusSlash".to_string(),
                icon: rsx!(Icon { icon: FaVirusSlash }),
            },

            IconName {
                name: "FaVirus".to_string(),
                icon: rsx!(Icon { icon: FaVirus }),
            },

            IconName {
                name: "FaViruses".to_string(),
                icon: rsx!(Icon { icon: FaViruses }),
            },

            IconName {
                name: "FaVoicemail".to_string(),
                icon: rsx!(Icon { icon: FaVoicemail }),
            },

            IconName {
                name: "FaVolcano".to_string(),
                icon: rsx!(Icon { icon: FaVolcano }),
            },

            IconName {
                name: "FaVolleyball".to_string(),
                icon: rsx!(Icon { icon: FaVolleyball }),
            },

            IconName {
                name: "FaVolumeHigh".to_string(),
                icon: rsx!(Icon { icon: FaVolumeHigh }),
            },

            IconName {
                name: "FaVolumeLow".to_string(),
                icon: rsx!(Icon { icon: FaVolumeLow }),
            },

            IconName {
                name: "FaVolumeOff".to_string(),
                icon: rsx!(Icon { icon: FaVolumeOff }),
            },

            IconName {
                name: "FaVolumeXmark".to_string(),
                icon: rsx!(Icon { icon: FaVolumeXmark }),
            },

            IconName {
                name: "FaVrCardboard".to_string(),
                icon: rsx!(Icon { icon: FaVrCardboard }),
            },

            IconName {
                name: "FaW".to_string(),
                icon: rsx!(Icon { icon: FaW }),
            },

            IconName {
                name: "FaWalkieTalkie".to_string(),
                icon: rsx!(Icon { icon: FaWalkieTalkie }),
            },

            IconName {
                name: "FaWallet".to_string(),
                icon: rsx!(Icon { icon: FaWallet }),
            },

            IconName {
                name: "FaWandMagicSparkles".to_string(),
                icon: rsx!(Icon { icon: FaWandMagicSparkles }),
            },

            IconName {
                name: "FaWandMagic".to_string(),
                icon: rsx!(Icon { icon: FaWandMagic }),
            },

            IconName {
                name: "FaWandSparkles".to_string(),
                icon: rsx!(Icon { icon: FaWandSparkles }),
            },

            IconName {
                name: "FaWarehouse".to_string(),
                icon: rsx!(Icon { icon: FaWarehouse }),
            },

            IconName {
                name: "FaWaterLadder".to_string(),
                icon: rsx!(Icon { icon: FaWaterLadder }),
            },

            IconName {
                name: "FaWater".to_string(),
                icon: rsx!(Icon { icon: FaWater }),
            },

            IconName {
                name: "FaWaveSquare".to_string(),
                icon: rsx!(Icon { icon: FaWaveSquare }),
            },

            IconName {
                name: "FaWeightHanging".to_string(),
                icon: rsx!(Icon { icon: FaWeightHanging }),
            },

            IconName {
                name: "FaWeightScale".to_string(),
                icon: rsx!(Icon { icon: FaWeightScale }),
            },

            IconName {
                name: "FaWheatAwnCircleExclamation".to_string(),
                icon: rsx!(Icon { icon: FaWheatAwnCircleExclamation }),
            },

            IconName {
                name: "FaWheatAwn".to_string(),
                icon: rsx!(Icon { icon: FaWheatAwn }),
            },

            IconName {
                name: "FaWheelchairMove".to_string(),
                icon: rsx!(Icon { icon: FaWheelchairMove }),
            },

            IconName {
                name: "FaWheelchair".to_string(),
                icon: rsx!(Icon { icon: FaWheelchair }),
            },

            IconName {
                name: "FaWhiskeyGlass".to_string(),
                icon: rsx!(Icon { icon: FaWhiskeyGlass }),
            },

            IconName {
                name: "FaWifi".to_string(),
                icon: rsx!(Icon { icon: FaWifi }),
            },

            IconName {
                name: "FaWind".to_string(),
                icon: rsx!(Icon { icon: FaWind }),
            },

            IconName {
                name: "FaWindowMaximize".to_string(),
                icon: rsx!(Icon { icon: FaWindowMaximize }),
            },

            IconName {
                name: "FaWindowMinimize".to_string(),
                icon: rsx!(Icon { icon: FaWindowMinimize }),
            },

            IconName {
                name: "FaWindowRestore".to_string(),
                icon: rsx!(Icon { icon: FaWindowRestore }),
            },

            IconName {
                name: "FaWineBottle".to_string(),
                icon: rsx!(Icon { icon: FaWineBottle }),
            },

            IconName {
                name: "FaWineGlassEmpty".to_string(),
                icon: rsx!(Icon { icon: FaWineGlassEmpty }),
            },

            IconName {
                name: "FaWineGlass".to_string(),
                icon: rsx!(Icon { icon: FaWineGlass }),
            },

            IconName {
                name: "FaWonSign".to_string(),
                icon: rsx!(Icon { icon: FaWonSign }),
            },

            IconName {
                name: "FaWorm".to_string(),
                icon: rsx!(Icon { icon: FaWorm }),
            },

            IconName {
                name: "FaWrench".to_string(),
                icon: rsx!(Icon { icon: FaWrench }),
            },

            IconName {
                name: "FaXRay".to_string(),
                icon: rsx!(Icon { icon: FaXRay }),
            },

            IconName {
                name: "FaX".to_string(),
                icon: rsx!(Icon { icon: FaX }),
            },

            IconName {
                name: "FaXmark".to_string(),
                icon: rsx!(Icon { icon: FaXmark }),
            },

            IconName {
                name: "FaXmarksLines".to_string(),
                icon: rsx!(Icon { icon: FaXmarksLines }),
            },

            IconName {
                name: "FaY".to_string(),
                icon: rsx!(Icon { icon: FaY }),
            },

            IconName {
                name: "FaYenSign".to_string(),
                icon: rsx!(Icon { icon: FaYenSign }),
            },

            IconName {
                name: "FaYinYang".to_string(),
                icon: rsx!(Icon { icon: FaYinYang }),
            },

            IconName {
                name: "FaZ".to_string(),
                icon: rsx!(Icon { icon: FaZ }),
            }
        ],
    }
}
