use logos::Logos;

#[derive[Logos, Debug, PartialEq]]
#[logos[(skip r"[ \t\n\f]")]]
pub enum Token{

    #[regex("[a-z]", |lex| lex.slice().chars().next().unwrap())]

    var(char),

    #[token(r"\neg")]

    not,

    #[token(r"\wedge")]

    and,

    #[token(r"\vee")]

    or,

    #[token(r"\rightarrow")]

    impl,

    #[token("(")]

    l_parentesis,

    #[token(")")]

    r_parentesis,
}

pub fn procces_input(atoms: &mut Vec<i32>, data: &mut Vec<char>, index_atoms: &mut Vec<i32>, n_variables: &mut i32, token_input: &[Token]){

    const DESP: i32 = 97;

    for token in token_input{

        match token{

            Token::var(c) =>{

                if((c as i32) - DESP == 0){

                    *n_variables = *n_variables + 1;

                }

                atoms[((c as i32) - DESP) as usize] = atoms[((c as i32) - DESP) as usize] + 1;
                
                data.push('0');
                index_atoms.push(c as i32)

            }

            Token::not => data.push('1'),
            Token::and => data.push('2'),
            Token::or => data.push('3'),
            Token::impl => data.push('4'),
            Token::l_parentesis => data.push('5'),
            Token::r_parentesis => data.push('6'),
        }

    }

}

fn main(){


    let expresion = "";

    let mut token_input = Token::lexer(expresion);
}