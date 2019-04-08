use rkaley::lexer::Lexer;
use rkaley::parser::Parser;

fn main() {
    let sample = r#"
    # print a hello world
    create game
    game's width = 500
    game's height = 600
    in game
      print "hello world!"
    "#;
    // let lex = Lexer::new(sample);
    // lex.for_each(|tok| println!("tok: {:?}", tok));
    let par = &mut Parser::from_source(sample);
    let ast = &mut par.parse_definition().unwrap();
    println!("ast: {:?}", ast);
}
