use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]")]
pub enum Token{

    #[regex("[a-z]", |lex| lex.slice().chars().next().unwrap())]

    Var(char),

    #[token(r"\neg")]

    Not,

    #[token(r"\wedge")]

    And,

    #[token(r"\vee")]

    Or,

    #[token(r"\rightarrow")]

    Imp,

    #[token("(")]

    LParentesis,

    #[token(")")]

    RParentesis,
}

pub fn procces_input(data: &mut Vec<i32>, index_atoms: &mut Vec<i32>, n_variables: &mut i32, token_input: &[Token]){

    for token in token_input{

        match token{

            Token::Var(c) =>{

                if !index_atoms.contains(&(*c as i32)) {

                    *n_variables = *n_variables + 1;

                }

                data.push(0);
                index_atoms.push(*c as i32)

            }

            Token::Not => data.push(1),
            Token::And => data.push(2),
            Token::Or => data.push(3),
            Token::Imp => data.push(4),
            Token::LParentesis => data.push(5),
            Token::RParentesis => data.push(6),
        }

    }

}

fn polaca_inversa(input: &[i32], index_atoms: &mut Vec<i32>) -> Vec<i32>{

    let mut stack: Vec<i32> = Vec::new();
    let mut output: Vec<i32> = Vec::new();
    let mut index_atomsP: Vec<i32> = Vec::new();

    let mut ii: usize = 0;
    let mut io: usize = 0;
    let mut n_p_open: usize = 0;
    let mut p_open: bool = false;

    for t_i in input{

        if(*t_i == 0){

            /*
                Si t_i  es variable, entonces
                insertar la variable en el output
                
            */

            index_atomsP.push(index_atoms[ii]);
            output.push(*t_i);

            ii += 1;

        }else if(!p_open && *t_i == 5){

            /*

                Si no hay parentesis abiertos y t_i es (, entonces
                apilar el (, p_open = True y n_p_open + 1

            */

            stack.push(*t_i);
            p_open = true;
            n_p_open += 1;

        }else if(p_open && *t_i != 6){

            if(*t_i == 5){

                n_p_open += 1;

            }

            stack.push(*t_i);

        }else if(*t_i == 6){

            while(!stack.is_empty() && stack.last().copied().unwrap_or(-1) != 5){

                output.push(stack.pop().unwrap());

            }

            stack.pop();
            n_p_open -= 1;

            if(n_p_open == 0){

                p_open = false;

            }

        }else{

            if(!stack.is_empty() && stack.last().copied().unwrap_or(0) <= *t_i){

                output.push(stack.pop().unwrap());
                stack.push(*t_i);

            }else{

                stack.push(*t_i);

            }

        }

    }

    while(!stack.is_empty()){

        output.push(stack.pop().unwrap());

    }

    *index_atoms = index_atomsP;

    return output

}

fn main(){


    let expresion = r"( ( p \rightarrow q ) \wedge ( q \rightarrow r) ) \rightarrow ( p \rightarrow r)";

    let mut data: Vec<i32> = Vec::new();
    let mut index_atoms: Vec<i32> = Vec::new();

    let mut n_variables: i32 = 0;

    let token_input: Vec<Token> = Token::lexer(expresion).collect::<Result<Vec<Token>, _>>().unwrap();

    procces_input(&mut data, &mut index_atoms, &mut n_variables, &token_input);

    for i in &data {

        print!("{} ", i);

    }

    println!("");

    let mut output: Vec<i32> = polaca_inversa(&data, &mut index_atoms);

    for i in &output {

        print!("{} ", i);

    }

    println!("");

}