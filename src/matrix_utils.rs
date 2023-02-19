use std::collections::HashMap;
use std::fs;
use std::process::exit;
use lazy_static::lazy_static;
use matrix_sdk::attachment::AttachmentConfig;
use matrix_sdk::room::{Joined};
use matrix_sdk::ruma::events::MessageLikeEvent;
use matrix_sdk::ruma::events::room::message::{RoomMessageEventContent};

pub type Command = fn(room: Joined, data: String);
trait CommandFn{
    fn get_command_name() -> String;
    fn handle_message(room: Joined, _data: String);
}

pub struct Ping{}
impl CommandFn for Ping{
    fn get_command_name() -> String{
        "ping".to_owned()
    }
    fn handle_message(room: Joined, _data: String) {
        tokio::spawn(async move {
            let content = RoomMessageEventContent::text_plain("🏓 pong 🏓");
                room.send(content, None).await.expect("Pong failed!");
        });
    }
}

/*
pub struct Halt{}
impl CommandFn for Halt{
    fn get_command_name() -> String{
        "halt".to_owned()
    }
    fn handle_message(room: Joined, _data: String) {
        tokio::spawn(async move {
            let content = RoomMessageEventContent::text_plain("Bye! 👋");
            room.send(content, None).await.expect("Bye failed!");
            exit(0);
        });
    }
}*/

pub struct Unknown{}
impl CommandFn for Unknown{
    fn get_command_name() -> String{
        "unknown".to_owned()
    }
    fn handle_message(room: Joined, _data: String) {
        tokio::spawn(async move {
            // Do nothing on unknown command
        });
    }
}

pub struct Handler {
    command_list: HashMap<String,  Command>
}

impl Handler{
    pub fn get_command(&self, name: &str) -> &Command{
        self.command_list.get(name).unwrap_or(&(Unknown::handle_message as Command))
    }
}


pub struct WhereIs {}
impl CommandFn for WhereIs {
    fn get_command_name() -> String {"whereis".to_owned()}
    fn handle_message(room: Joined, data: String) {
        
        tokio::spawn(async move {
            let mut content = RoomMessageEventContent::text_markdown("`Item not found.`");
            let mut sendAttachment = false;
            let mut imagePath = "";
            let mut imageHeader = "";
            match data.trim().to_lowercase().as_str(){
                // misc links
                "faq" => content = RoomMessageEventContent::text_markdown("**Community Updates FaQ**: https://am2r-community-developers.github.io/DistributionCenter/faq"),
                "changelog" => content = RoomMessageEventContent::text_markdown("**Cumulative AM2R Changelog**: https://am2r-community-developers.github.io/DistributionCenter/changelog"),
                "doc"|"doctorm64"|"milton" => content = RoomMessageEventContent::text_markdown("`Creating games at Moon Studios!` https://www.orithegame.com/"),
                "ridley"|"kraid"|"croc"|"crocomire" => content = RoomMessageEventContent::text_markdown("`Waiting to challenge Samus in Metroid: Confrontation!` https://metroid2remake.blogspot.com/p/metroid-confrontation.html"),
                // TODO: needs a link
                "druid"|"druidvorse" => content = RoomMessageEventContent::text_markdown("`Spaceboosting across SR388 on YouTube and Twitch!`"),
                "sabre320"|"sabre" => content = RoomMessageEventContent::text_markdown("`Exploring the history of Dinosaur Planet!`"),
                "syphonzoa" => content = RoomMessageEventContent::text_markdown("`Feasting on the endless buffet of Hornoads in AM2R: The Horde!` https://github.com/Hornoads/AM2R-The-Horde-Multitroid/releases"),
                
                // alternatively use the image thingy for 1.1
                "am2r"|"am2r_11"|"am2r 1.1" => content = RoomMessageEventContent::text_markdown("`Once on the internet, always on the internet. Let Google be your guide.`"),
                
                // items
                // HACK: these *should* upload proper images. However, since our t2bot media repo is very unstable, we send discord links instead.
                "bomb"|"bombs" => content = RoomMessageEventContent::text_markdown("https://cdn.discordapp.com/attachments/509717926807601182/1076652269543620618/whereis_bombs.gif"),
                //"spider"|"spider ball"|"spiderball" => {sendAttachment = true; imageHeader = "Spider Ball"; imagePath = "/home/narr/Dokumente/gitRepos/matrixAm2rBot/whereis/whereis_spiderball.gif"},
                "spider"|"spider ball"|"spiderball" => content = RoomMessageEventContent::text_markdown("https://cdn.discordapp.com/attachments/509717926807601182/1076652295967748217/whereis_spiderball.gif"),
                "spring"|"springball"|"spring ball"|"jumpball"|"jump ball" => content = RoomMessageEventContent::text_markdown("https://cdn.discordapp.com/attachments/509717926807601182/1076652296357822577/whereis_springball.gif"),
                "screw"|"screw attack" => content = RoomMessageEventContent::text_markdown("https://cdn.discordapp.com/attachments/509717926807601182/1076652272701939882/whereis_screwattack.gif"),
                "varia"|"varia suit" => content = RoomMessageEventContent::text_markdown("https://cdn.discordapp.com/attachments/509717926807601182/1076652297825829025/whereis_variasuit.gif"),
                "space"|"spacejump"|"space jump" => content = RoomMessageEventContent::text_markdown("https://cdn.discordapp.com/attachments/509717926807601182/1076652294717841489/whereis_spacejump.gif"),
                "speed"|"speedbooster"|"speed booster" => content = RoomMessageEventContent::text_markdown("https://cdn.discordapp.com/attachments/509717926807601182/1076652295556702258/whereis_speedbooster.gif"),
                "hijump"|"highjump"|"hi jump"| "high jump" => content = RoomMessageEventContent::text_markdown("https://cdn.discordapp.com/attachments/509717926807601182/1076652270965497876/whereis_highjump.gif"),
                "gravity"|"gravity suit" => content = RoomMessageEventContent::text_markdown("https://cdn.discordapp.com/attachments/509717926807601182/1076652270407667812/whereis_gravitysuit.gif"),
                "charge"|"chargebeam"|"charge beam" => content = RoomMessageEventContent::text_markdown("https://cdn.discordapp.com/attachments/509717926807601182/1076652269988225095/whereis_chargebeam.gif"),
                "ice"|"icebeam"|"ice beam" => content = RoomMessageEventContent::text_markdown("https://cdn.discordapp.com/attachments/509717926807601182/1076652271464611840/whereis_icebeam.gif"),
                "wave"|"wavebeam"|"wave beam" => content = RoomMessageEventContent::text_markdown("https://cdn.discordapp.com/attachments/509717926807601182/1076652300002656317/whereis_wavebeam.gif"),
                "spazer"|"spazerbeam"|"spazer beam" => content = RoomMessageEventContent::text_markdown("https://cdn.discordapp.com/attachments/509717926807601182/1076652295078555739/whereis_spazerbeam.gif"),
                "plasma"|"plasmabeam"|"plasma beam" => content = RoomMessageEventContent::text_markdown("https://cdn.discordapp.com/attachments/509717926807601182/1076652271909228646/whereis_plasmabeam.gif"),
                "super"|"supers"|"super missile" => content = RoomMessageEventContent::text_markdown("https://cdn.discordapp.com/attachments/509717926807601182/1076652296890490931/whereis_supermissiles.gif"),
                //"test" => content = RoomMessageEventContent::text_markdown("<img data-mx-emoticon=\"\" src=\"mxc://matrix.org/oOMqTxQVRpssDfBBWvJjVrhf\" alt=\":derp:\" title=\":derp:\" height=\"32\" vertical-align=\"middle\" /> **asdfasdf**"),
                "pbomb"|"pbombs"|"powerbomb"|"powerbombs"|"power bomb"|"power bombs" => content = RoomMessageEventContent::text_markdown("https://cdn.discordapp.com/attachments/509717926807601182/1076652272324444180/whereis_powerbombs.gif"),
                
                _ => content = content
            } 
            
            if sendAttachment {
                let mut image = fs::read(imagePath).unwrap();
                room.send_attachment(
                    imageHeader,
                    &mime::IMAGE_GIF,
                    &image,
                    AttachmentConfig::new(),
                ).await.expect("upload failed");
            }
            else { 
                room.send(content, None).await.expect("whereis failed!");
            }
        });
    }
}

pub struct FAQ {}
impl CommandFn for FAQ {
    fn get_command_name() -> String {
        "faq".to_owned()
    }

    fn handle_message(room: Joined, _data: String) {
        tokio::spawn(async move {
            let content = RoomMessageEventContent::text_markdown("**Community Updates FaQ**: https://am2r-community-developers.github.io/DistributionCenter/faq");
            room.send(content, None).await.expect("Faq failed!");
        });
    }
}

pub struct Changelog {}
impl CommandFn for Changelog {
    fn get_command_name() -> String {
        "changelog".to_owned()
    }

    fn handle_message(room: Joined, _data: String) {
        tokio::spawn(async move {
            let content = RoomMessageEventContent::text_markdown("**Cumulative AM2R Changelog**: https://am2r-community-developers.github.io/DistributionCenter/changelog");
            room.send(content, None).await.expect("Changelog failed!");
        });
    }
}

lazy_static! {
    pub static ref HANDY: Handler = {
        let mut com = HashMap::<String, Command>::new();
        com.insert(WhereIs::get_command_name(), WhereIs::handle_message);
        com.insert(FAQ::get_command_name(), FAQ::handle_message);
        com.insert(Changelog::get_command_name(), Changelog::handle_message);
        com.insert(Ping::get_command_name() , Ping::handle_message);
        //com.insert(Halt::get_command_name() , Halt::handle_message);
        Handler{command_list: com}
    };
}
