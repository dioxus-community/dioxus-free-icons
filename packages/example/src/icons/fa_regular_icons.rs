
use crate::{IconSet, IconName};
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_regular_icons::*;
use dioxus_free_icons::Icon;


pub fn get_icon_set() -> IconSet{
    IconSet {
        code: "fa_regular_icons".to_string(),
        name: "Font Awesome".to_string(),
        url: "https://fontawesome.com/".to_string(),
        license: "CC BY 4.0 License".to_string(),
        license_url: "https://creativecommons.org/licenses/by/4.0/".to_string(),
        version: "6.1.1".to_string(),
        source_url: "https://github.com/FortAwesome/Font-Awesome/tree/6.1.1".to_string(),
        icons: vec![

            IconName {
                name: "FaAddressBook".to_string(),
                icon: rsx!(Icon { icon: FaAddressBook }),
            },

            IconName {
                name: "FaAddressCard".to_string(),
                icon: rsx!(Icon { icon: FaAddressCard }),
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
                name: "FaBookmark".to_string(),
                icon: rsx!(Icon { icon: FaBookmark }),
            },

            IconName {
                name: "FaBuilding".to_string(),
                icon: rsx!(Icon { icon: FaBuilding }),
            },

            IconName {
                name: "FaCalendarCheck".to_string(),
                icon: rsx!(Icon { icon: FaCalendarCheck }),
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
                name: "FaCalendarXmark".to_string(),
                icon: rsx!(Icon { icon: FaCalendarXmark }),
            },

            IconName {
                name: "FaCalendar".to_string(),
                icon: rsx!(Icon { icon: FaCalendar }),
            },

            IconName {
                name: "FaChartBar".to_string(),
                icon: rsx!(Icon { icon: FaChartBar }),
            },

            IconName {
                name: "FaChessBishop".to_string(),
                icon: rsx!(Icon { icon: FaChessBishop }),
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
                name: "FaCircleCheck".to_string(),
                icon: rsx!(Icon { icon: FaCircleCheck }),
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
                name: "FaCircleLeft".to_string(),
                icon: rsx!(Icon { icon: FaCircleLeft }),
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
                name: "FaCircleQuestion".to_string(),
                icon: rsx!(Icon { icon: FaCircleQuestion }),
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
                name: "FaClipboard".to_string(),
                icon: rsx!(Icon { icon: FaClipboard }),
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
                name: "FaCommentDots".to_string(),
                icon: rsx!(Icon { icon: FaCommentDots }),
            },

            IconName {
                name: "FaComment".to_string(),
                icon: rsx!(Icon { icon: FaComment }),
            },

            IconName {
                name: "FaComments".to_string(),
                icon: rsx!(Icon { icon: FaComments }),
            },

            IconName {
                name: "FaCompass".to_string(),
                icon: rsx!(Icon { icon: FaCompass }),
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
                name: "FaCreditCard".to_string(),
                icon: rsx!(Icon { icon: FaCreditCard }),
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
                name: "FaEyeSlash".to_string(),
                icon: rsx!(Icon { icon: FaEyeSlash }),
            },

            IconName {
                name: "FaEye".to_string(),
                icon: rsx!(Icon { icon: FaEye }),
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
                name: "FaFileAudio".to_string(),
                icon: rsx!(Icon { icon: FaFileAudio }),
            },

            IconName {
                name: "FaFileCode".to_string(),
                icon: rsx!(Icon { icon: FaFileCode }),
            },

            IconName {
                name: "FaFileExcel".to_string(),
                icon: rsx!(Icon { icon: FaFileExcel }),
            },

            IconName {
                name: "FaFileImage".to_string(),
                icon: rsx!(Icon { icon: FaFileImage }),
            },

            IconName {
                name: "FaFileLines".to_string(),
                icon: rsx!(Icon { icon: FaFileLines }),
            },

            IconName {
                name: "FaFilePdf".to_string(),
                icon: rsx!(Icon { icon: FaFilePdf }),
            },

            IconName {
                name: "FaFilePowerpoint".to_string(),
                icon: rsx!(Icon { icon: FaFilePowerpoint }),
            },

            IconName {
                name: "FaFileVideo".to_string(),
                icon: rsx!(Icon { icon: FaFileVideo }),
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
                name: "FaFlag".to_string(),
                icon: rsx!(Icon { icon: FaFlag }),
            },

            IconName {
                name: "FaFloppyDisk".to_string(),
                icon: rsx!(Icon { icon: FaFloppyDisk }),
            },

            IconName {
                name: "FaFolderClosed".to_string(),
                icon: rsx!(Icon { icon: FaFolderClosed }),
            },

            IconName {
                name: "FaFolderOpen".to_string(),
                icon: rsx!(Icon { icon: FaFolderOpen }),
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
                name: "FaFutbol".to_string(),
                icon: rsx!(Icon { icon: FaFutbol }),
            },

            IconName {
                name: "FaGem".to_string(),
                icon: rsx!(Icon { icon: FaGem }),
            },

            IconName {
                name: "FaHandBackFist".to_string(),
                icon: rsx!(Icon { icon: FaHandBackFist }),
            },

            IconName {
                name: "FaHandLizard".to_string(),
                icon: rsx!(Icon { icon: FaHandLizard }),
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
                name: "FaHandSpock".to_string(),
                icon: rsx!(Icon { icon: FaHandSpock }),
            },

            IconName {
                name: "FaHand".to_string(),
                icon: rsx!(Icon { icon: FaHand }),
            },

            IconName {
                name: "FaHandshake".to_string(),
                icon: rsx!(Icon { icon: FaHandshake }),
            },

            IconName {
                name: "FaHardDrive".to_string(),
                icon: rsx!(Icon { icon: FaHardDrive }),
            },

            IconName {
                name: "FaHeart".to_string(),
                icon: rsx!(Icon { icon: FaHeart }),
            },

            IconName {
                name: "FaHospital".to_string(),
                icon: rsx!(Icon { icon: FaHospital }),
            },

            IconName {
                name: "FaHourglass".to_string(),
                icon: rsx!(Icon { icon: FaHourglass }),
            },

            IconName {
                name: "FaIdBadge".to_string(),
                icon: rsx!(Icon { icon: FaIdBadge }),
            },

            IconName {
                name: "FaIdCard".to_string(),
                icon: rsx!(Icon { icon: FaIdCard }),
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
                name: "FaKeyboard".to_string(),
                icon: rsx!(Icon { icon: FaKeyboard }),
            },

            IconName {
                name: "FaLemon".to_string(),
                icon: rsx!(Icon { icon: FaLemon }),
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
                name: "FaMap".to_string(),
                icon: rsx!(Icon { icon: FaMap }),
            },

            IconName {
                name: "FaMessage".to_string(),
                icon: rsx!(Icon { icon: FaMessage }),
            },

            IconName {
                name: "FaMoneyBill1".to_string(),
                icon: rsx!(Icon { icon: FaMoneyBill1 }),
            },

            IconName {
                name: "FaMoon".to_string(),
                icon: rsx!(Icon { icon: FaMoon }),
            },

            IconName {
                name: "FaNewspaper".to_string(),
                icon: rsx!(Icon { icon: FaNewspaper }),
            },

            IconName {
                name: "FaNoteSticky".to_string(),
                icon: rsx!(Icon { icon: FaNoteSticky }),
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
                name: "FaPaperPlane".to_string(),
                icon: rsx!(Icon { icon: FaPaperPlane }),
            },

            IconName {
                name: "FaPaste".to_string(),
                icon: rsx!(Icon { icon: FaPaste }),
            },

            IconName {
                name: "FaPenToSquare".to_string(),
                icon: rsx!(Icon { icon: FaPenToSquare }),
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
                name: "FaRegistered".to_string(),
                icon: rsx!(Icon { icon: FaRegistered }),
            },

            IconName {
                name: "FaShareFromSquare".to_string(),
                icon: rsx!(Icon { icon: FaShareFromSquare }),
            },

            IconName {
                name: "FaSnowflake".to_string(),
                icon: rsx!(Icon { icon: FaSnowflake }),
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
                name: "FaSquareFull".to_string(),
                icon: rsx!(Icon { icon: FaSquareFull }),
            },

            IconName {
                name: "FaSquareMinus".to_string(),
                icon: rsx!(Icon { icon: FaSquareMinus }),
            },

            IconName {
                name: "FaSquarePlus".to_string(),
                icon: rsx!(Icon { icon: FaSquarePlus }),
            },

            IconName {
                name: "FaSquare".to_string(),
                icon: rsx!(Icon { icon: FaSquare }),
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
                name: "FaStar".to_string(),
                icon: rsx!(Icon { icon: FaStar }),
            },

            IconName {
                name: "FaSun".to_string(),
                icon: rsx!(Icon { icon: FaSun }),
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
                name: "FaTrashCan".to_string(),
                icon: rsx!(Icon { icon: FaTrashCan }),
            },

            IconName {
                name: "FaUser".to_string(),
                icon: rsx!(Icon { icon: FaUser }),
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
            }
        ],
    }
}
