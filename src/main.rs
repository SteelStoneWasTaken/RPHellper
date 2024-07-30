/*-----------------------------------------------------------------------------------------------+
|                                                                                                |
|    /$$$$$$$  /$$$$$$$   /$$$$$$  /$$   /$$           /$$ /$$                                   |
|    | $$__  $$| $$__  $$ /$$__  $$| $$  | $$          | $$| $$                                  |
|    | $$  \ $$| $$  \ $$| $$  \__/| $$  | $$  /$$$$$$ | $$| $$  /$$$$$$   /$$$$$$  /$$$$$$      |
|    | $$$$$$$/| $$$$$$$/| $$ /$$$$| $$$$$$$$ /$$__  $$| $$| $$ /$$__  $$ /$$__  $$/$$__  $$     |
|    | $$__  $$| $$____/ | $$|_  $$| $$__  $$| $$$$$$$$| $$| $$| $$  \ $$| $$$$$$$$| $$  \__/    |
|    | $$  \ $$| $$      | $$  \ $$| $$  | $$| $$_____/| $$| $$| $$  | $$| $$_____/| $$          |
|    | $$  | $$| $$      |  $$$$$$/| $$  | $$|  $$$$$$$| $$| $$| $$$$$$$/|  $$$$$$$| $$          |
|    |__/  |__/|__/       \______/ |__/  |__/ \_______/|__/|__/| $$____/  \_______/|__/          |
|                                                             | $$                               |
|        A Discord bot to help RPGs by Steely (SteelStone)    | $$    1.0.0-BETA                 |
|                                                             |__/                               |
|                                                                                                |
| * Art made in: https://www.asciiart.eu/text-to-ascii-art (Using "Big Money-ne")                |
+------------------------------------------------------------------------------------------------*/

use std::env;
use rand::distributions::{Distribution, Uniform};

use serenity::all::Message;
use serenity::async_trait;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Variáveis.
        let mut index = 0;
        let mut output: Vec<String> = Vec::new();
        let mut input: Vec<String> = msg.content
            .chars().map(|f| f.to_string()).collect();
        
        // Se a mensagem for dele mesmo ou for inteiro comentário, ignore.
        let bot_id = ctx.http.get_current_user().await.unwrap().id;
        if msg.author.id == bot_id
        || msg.content.chars().next() == Some('\'')
        || msg.content.chars().next() == Some('"')
        {return;}
        
        /*-------------------------+
        | PROCURA COMANDOS RÁPIDOS |
        +--------------------------*/
        match msg.content.to_string().to_lowercase().as_str().trim() {
            "ping"|"ping!" => if let Err(why) = msg.reply_ping(&ctx.http, "Pong!")
                .await {println!("Error sending message: {why:?}");},
            "moeda"|"coin" => {
                let die = Uniform::from(1..3).sample(&mut rand::thread_rng());
                if die == 1 {
                    if let Err(why) = msg.reply_ping(&ctx.http, "**Cara**")
                        .await {println!("Error sending message: {why:?}");}
                }else {
                    if let Err(why) = msg.reply_ping(&ctx.http, "**Coroa**")
                        .await {println!("Error sending message: {why:?}");}
                }
                }
            "rphelp" => if let Err(why) = msg.reply_ping(&ctx.http, HELP)
                .await {println!("Error sending message: {why:?}");},
            "r1" => if let Err(why) = msg.reply_ping(&ctx.http, R1)
                .await {println!("Error sending message: {why:?}");},
            "r2" => if let Err(why) = msg.reply_ping(&ctx.http, R2)
                .await {println!("Error sending message: {why:?}");},
            "r3" => if let Err(why) = msg.reply_ping(&ctx.http, R3)
                .await {println!("Error sending message: {why:?}");},
            _ => ()
        }
        // Se a mensagem não tiver números, ignore.
        if !msg.content.replace(" ", "").chars().any(char::is_numeric){return;}
        /*-----------------------+
        | IDENTIFICA COMENTÁRIOS |
        +------------------------*/
        // Variáveis necessárias.
        let mut txt: Vec<String> = Vec::new();
        let mut to_txt = false;
        // Identifica comentários forçados pelo usuário
        // usando " ou '.
        while index < input.len(){
            if to_txt{
                txt.push(input[index].clone());
                    input.remove(index);
                    index -= 1;
            }
            match input[index].as_str() {
                "\""|"'" => {
                    to_txt = true;
                    input.remove(index);
                    index -= 1;
                },
                _ => ()
            }
            index +=1;
        }
        // Identifica se não for forçado pelo usuário
        // e, se forçado, apaga completamente partes
        // que atrapalhariam.
        while !input[input.len()-1].chars().all(char::is_numeric) && input[input.len()-1] != ")"{
            if to_txt{
                input.remove(input.len()-1);
            } else {
                txt.insert(0, input[input.len()-1].clone());
                input.remove(input.len()-1);
            }
        }
        /*-----------------------------+
        | PREPARA O INPUT PARA CÁLCULO |
        +-----------------------------*/
        // Remove algumas possibilidades de erro.
        if input.join("").chars().all(char::is_numeric)
        || input.join("").chars().all(char::is_alphabetic){
            return;
        }
        // Remove espaços.
        input.retain(|s| !s.trim().is_empty());
        input = input.iter().map(|s| s.to_lowercase()).collect();
        // Separa input em "numéricos" e "não numéricos"
        index = 0;
        let mut neo_input: Vec<String> = Vec::new();
        let mut tmp_input: Vec<String> = Vec::new();
        
        while index < input.len(){
            if input[index].chars().all(char::is_numeric)
            || input[index].contains(".")
            {
                tmp_input.push(input[index].clone());
            }else {
                if !tmp_input.is_empty() {
                    neo_input.push(tmp_input.join(""));
                    tmp_input = Vec::new();
                }
                neo_input.push(input[index].clone());
            }
            index +=1
        }
        if !tmp_input.is_empty() {
            neo_input.push(tmp_input.join(""));
        }
        // Abandone se hover comandos inexistentes.
        index = 0;
        while index < neo_input.len() {
            if !neo_input[index].chars().any(char::is_numeric)
            && !neo_input[index].contains(")")
            && !neo_input[index].contains("("){
                match neo_input[index].as_str() {
                    "g"|"l"|"a"|"d" => {
                        // Se o proximo carctere não for um número ou parêntese, desista.
                        if !neo_input[index + 1].chars().any(char::is_numeric)
                        && !neo_input[index + 1].contains("("){
                            return;
                        }
                    }
                    "+"|"-"|"*"|"/" => {
                        // Se o proximo carctere não for um número, dado, ou parêntese, desista.
                        if !neo_input[index + 1].chars().any(char::is_numeric)
                        && !neo_input[index + 1].contains("(")
                        && !neo_input[index + 1].contains("g")
                        && !neo_input[index + 1].contains("l")
                        && !neo_input[index + 1].contains("a")
                        && !neo_input[index + 1].contains("d")
                        && !neo_input[index + 1].contains("-")
                        {
                            return;
                        }
                    },
                    _ => return 
                }
            }
            index += 1;
        }
        // Arruma números negativos.
        let mut bugfix = false;
        index = 0;
        while index < neo_input.len() {
            if neo_input[index].contains("-"){
                if neo_input[index + 1].chars().any(char::is_numeric){
                    neo_input[index + 1].insert(0, '-');
                    neo_input[index] = String::from("+");
                    if index == neo_input.len() - 2{
                        neo_input.remove(index);
                        bugfix = true;
                    } else {
                        if index == 0
                        || neo_input[index - 1] == "("
                        || neo_input[index - 1] == "+"
                        || neo_input[index - 1] == "-"
                        || neo_input[index - 1] == "*"
                        || neo_input[index - 1] == "/"
                         {
                            neo_input.remove(index);
                        }
                    }
                }
            }
            index += 1;
        }
        if bugfix {
            neo_input.insert(neo_input.len() - 1, String::from("+"))
        }
        /*----------------+
        | FAZ OS CALCULOS |
        +-----------------*/
        neo_input.push(String::from(")"));
        neo_input.insert(0, String::from("("));
        let mut time = 1;
        loop {
            // Coloca multiplicação se sem sinal.
            index = 0;
            while index < neo_input.len() {
                if neo_input[index].chars().any(char::is_numeric)
                && index != 0{
                    if neo_input[index - 1].chars().any(char::is_numeric){
                        neo_input.insert(index, String::from("*"))
                    }
                }
                index += 1;
            }
            // Coloca 1 antes de comando de dados.
            index = 0;
            while index < neo_input.len() {
                match neo_input[index].as_str(){
                    "g"|"l"|"a"|"d" => {
                        if !neo_input[index - 1].chars().any(char::is_numeric){
                            neo_input.insert(index, String::from("1"))
                        }
                    }
                    _ => ()
                }
                index += 1;
            }
            // Separa o primeiro parêntese a ser resolvido.
            let mut to_solve: Vec<String> = Vec::new();
            index = 0;
            while neo_input[index] != ")" {
                to_solve.push(neo_input[index].clone());
                index += 1;
            }
            while to_solve.contains(&String::from("(")) {
                to_solve.remove(0);
            }
            output.push(String::from(format!("### {time}. (*{}*)\n", to_solve.join(""))));
            loop {
                // Resolve os dados se possível.
                index = 0;
                while index < to_solve.len(){
                    match to_solve[index].as_str() {
                        "g" => {
                            let faces: u32 = to_solve[index+1].parse().expect(ERR1);
                            let mut repeat: u32 = 1;
                            let mut result = 0;
                            if index != 0{
                                repeat = to_solve[index-1].parse().expect(ERR1);
                            }
                            output.push(String::from(format!("**{repeat}g{faces}** -> ")));
                            while repeat > 0 {
                                let die = Uniform::from(1..faces+1).sample(&mut rand::thread_rng());
                                if die > result{
                                    result = die
                                }
                                output.push(String::from(format!("[{die}]")));
                                if repeat != 1 {
                                    output.push(String::from(format!(" ")));
                                }
                                repeat -= 1
                            }
                            output.push(String::from(format!(" = **{result}**;\n")));
                            to_solve.remove(index + 1);
                            to_solve[index] = String::from(result.to_string());
                            if index != 0{
                                to_solve.remove(index - 1);
                                index -= 1;
                            }
                            output.push(String::from(format!("*{}*\n", to_solve.join(""))));
                        }
                        "l" => {
                            let faces: u32 = to_solve[index+1].parse().expect(ERR1);
                            let mut repeat: u32 = 1;
                            let mut result = faces;
                            if index != 0{
                                repeat = to_solve[index-1].parse().expect(ERR1);
                            }
                            output.push(String::from(format!("**{repeat}l{faces}** -> ")));
                            while repeat > 0 {
                                let die = Uniform::from(1..faces+1).sample(&mut rand::thread_rng());
                                if die < result{
                                    result = die
                                }
                                output.push(String::from(format!("[{die}]")));
                                if repeat != 1 {
                                    output.push(String::from(format!(" ")));
                                }
                                repeat -= 1
                            }
                            output.push(String::from(format!(" = **{result}**;\n")));
                            to_solve.remove(index + 1);
                            to_solve[index] = String::from(result.to_string());
                            if index != 0{
                                to_solve.remove(index - 1);
                                index -= 1;
                            }
                            output.push(String::from(format!("*{}*\n", to_solve.join(""))));
                        }
                        "a" => {
                            let faces: u32 = to_solve[index+1].parse().expect(ERR1);
                            let mut repeat: u32 = 1;
                            let mut result = 0;
                            if index != 0{
                                repeat = to_solve[index-1].parse().expect(ERR1);
                            }
                            output.push(String::from(format!("**TEMP! {repeat}a{faces}** -> ")));
                            while repeat > 0 {
                                let die = Uniform::from(1..faces+1).sample(&mut rand::thread_rng());
                                result += die;
                                output.push(String::from(format!("[{die}]")));
                                if repeat != 1 {
                                    output.push(String::from(format!(" + ")));
                                }
                                repeat -= 1
                            }
                            output.push(String::from(format!(" = **{result}**;\n")));
                            to_solve.remove(index + 1);
                            to_solve[index] = String::from(result.to_string());
                            if index != 0{
                                to_solve.remove(index - 1);
                                index -= 1;
                            }
                            output.push(String::from(format!("*{}*\n", to_solve.join(""))));
                        }
                        "d" => {
                            let faces: u32 = to_solve[index+1].parse().expect(ERR1);
                            let mut repeat: u32 = 1;
                            let mut result = 0;
                            if index != 0{
                                repeat = to_solve[index-1].parse().expect(ERR1);
                            }
                            output.push(String::from(format!("**{repeat}d{faces}** -> ")));
                            while repeat > 0 {
                                let die = Uniform::from(1..faces+1).sample(&mut rand::thread_rng());
                                result += die;
                                output.push(String::from(format!("[{die}]")));
                                if repeat != 1 {
                                    output.push(String::from(format!(" + ")));
                                }
                                repeat -= 1
                            }
                            output.push(String::from(format!(" = **{result}**;\n")));
                            to_solve.remove(index + 1);
                            to_solve[index] = String::from(result.to_string());
                            if index != 0{
                                to_solve.remove(index - 1);
                                index -= 1;
                            }
                            output.push(String::from(format!("*{}*\n", to_solve.join(""))));
                        }
                        _ => ()
                    }
                    index += 1;
                }
                // Resolve multiplicação e divisão se possível.
                index = 0;
                while index < to_solve.len(){
                    match to_solve[index].as_str() {
                        "*" => {
                            if to_solve[index + 1].chars().any(char::is_numeric)
                            && to_solve[index - 1].chars().any(char::is_numeric){
                                let n1: f64 = to_solve[index - 1].parse().expect("msg");
                                let n2: f64 = to_solve[index + 1].parse().expect("msg");
                                let result = n1*n2;
                                output.push(String::from(format!("**{n1} \\* {n2}** = **{result}**;\n")));
                                
                                to_solve.remove(index + 1);
                                to_solve[index] = String::from(result.to_string());
                                to_solve.remove(index - 1);
                                index -= 1;
                                output.push(String::from(format!("*{}*\n", to_solve.join(""))));
                            }
                        }
                        "/" => {
                            if to_solve[index + 1].chars().any(char::is_numeric)
                            && to_solve[index - 1].chars().any(char::is_numeric){
                                let n1: f64 = to_solve[index - 1].parse().expect("msg");
                                let n2: f64 = to_solve[index + 1].parse().expect("msg");
                                let result = n1/n2;
                                output.push(String::from(format!("**{n1} / {n2}** = **{result}**;\n")));
                                
                                to_solve.remove(index + 1);
                                to_solve[index] = String::from(result.to_string());
                                to_solve.remove(index - 1);
                                index -= 1;
                                output.push(String::from(format!("*{}*\n", to_solve.join(""))));
                            }
                        }
                        _ => ()
                    }
                    index += 1;
                }
                // Resolve adição e subtração.
                index = 0;
                while index < to_solve.len(){
                    if to_solve[index] == "+"{
                        if to_solve[index + 1].chars().any(char::is_numeric)
                        && to_solve[index - 1].chars().any(char::is_numeric){
                            let n1: f64 = to_solve[index - 1].parse().expect("msg");
                            let n2: f64 = to_solve[index + 1].parse().expect("msg");
                            let result = n1+n2;
                            output.push(String::from(format!("**{n1}** + **{n2}** = **{result}**;\n")));
                            
                            to_solve.remove(index + 1);
                            to_solve[index] = String::from(result.to_string());
                            to_solve.remove(index - 1);
                            index -= 1;
                            output.push(String::from(format!("*{}*\n", to_solve.join(""))));
                        }
                    }
                    index += 1;
                }
                // quando o resultado for um único número, quebre o loop.
                if to_solve.len() == 1{
                    index = 0;
                    while index < neo_input.len(){
                        if neo_input[index] == ")"{
                            neo_input[index] = String::from(to_solve.join(""));
                            loop {
                                if neo_input[index - 1].contains("("){
                                    neo_input.remove(index - 1); break;
                                } else {
                                    neo_input.remove(index - 1); index -= 1;
                                }
                            }
                            break;
                        }
                        index += 1;
                    }
                    break;
                }
            }
            
            
            time += 1;
            // quando o resultado for um único número, quebre o loop.
            if neo_input.len() == 1 {
                break;
            }
        }
        
        
        /*--------------------+
        | ENTREGA O RESULTADO |
        +---------------------*/
        output.push("------------------\n".to_string());
        output.push(String::from(format!("**{}**", neo_input.join(""))));
        if !txt.is_empty(){
            output.push(" ".to_string());
            output.push(String::from(txt.join("").trim()));
        }
        
        // Envia o resultado.
        println!("\n
            \nResposta fornecida ao usuário:\
            \n------------------------------\
            \n{}\
            \n------------------------------\
            ",output.join(""));
        if let Err(why) = msg.reply_ping(&ctx.http, format!("{}",output.join("")))
            .await {println!("Error sending message: {why:?}");}
        
        
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    println!("Ready!");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Start listening for events by starting a single shar
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

/*----------+
| MENSAGENS |
+-----------*/
// Aqui estão as mensagens que podem ser exibidas.
const HELP: &str = "
# RPHellp
*Autor: Steely (SteelStone)*
-# \"RP\" de \"Role Playing ~~Game~~ \"
-# \"Hellp\" de \"Helper (Ajudante) ~~E pq foi um *\"hell\"* fazer isso~~ (zoera :D) \"


**Para respostas sobre as seguintes perguntas, digite o código respectivo (Sem áspas)**
**\"r1\"** Por que meu comando não está funcionando?
**\"r2\"** Como jogo um dado? Como funciona?
**\"r3\"** Como funcionam os comentários?
";
const R1: &str = "
## \"Por que meu comando não está funcionando?\"
**Existem alguns motivos para seu comando não estar funcionando corretamente. São eles:**

BlaBlaBla...
";
const R2: &str = "
## \"Como jogo um dado? Como funciona?\"
**Existem muitas formas de se jogar dados. São elas:**

Se quiser jogar **um dado único**, use o comando \"**d**\" antes de um número.
Te fornecerá um resultado entre 1 e o número posterior.

BlaBlaBla...
";
const R3: &str = "
## \"Como funcionam os comentários?\"
**Os comentários podem ser identificados de duas formas: Automática e Forçada.**
**Segue o funcionamento de cada forma:**

BlaBlaBla...
";
const ERR1: &str = "
Nenhum comando de dados aceita números negativos ou fracionários.
";