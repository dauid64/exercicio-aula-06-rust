#![allow(unused)]
mod escritorio;

use std::collections::HashMap;

use escritorio::{da_aumento, promove, Beneficio, DadosFuncionais, DadosPessoais};
use escritorio::{Assalariado, AtributosFuncionais};

use crate::escritorio::Funcionario::{self, *};
use crate::escritorio::{desconta_falta, FuncionarioRegular, Gerente};

fn main() {
    execute();
    //arrays_vs_vectors_vs_tuples();
}

fn informa_funcionario_do_mes() -> i32 {
    //Leu do banco de dados, aplicou regras para enquadramento
    123
}

fn informa_funcionarios_faltantes() -> Vec<i32> {
    vec![123, 321]
}

fn execute() {
    let funcionario1 = Reg(FuncionarioRegular::new(
        DadosPessoais::new("Fulano".to_string(), 30),
        DadosFuncionais::new(1000.0),
        Beneficio::new(100.0, 100.0),
    ));
    let funcionario2 = Reg(FuncionarioRegular::new(
        DadosPessoais::new("Sicrano".to_string(), 30),
        DadosFuncionais::new(1000.0),
        Beneficio::new(100.0, 100.0),
    ));

    let id_f1 = 123;
    let id_f2 = 321;

    let mut registros: HashMap<i32, Funcionario> = HashMap::new();
    registros.insert(id_f1, funcionario1);
    registros.insert(id_f2, funcionario2);

    let id_funcionario_do_mes = informa_funcionario_do_mes();

    if let Some(Reg(funcionario_do_mes)) = registros.get(&id_funcionario_do_mes) {
        println!(
            "O funcionario do mes é {}",
            funcionario_do_mes.dados_pessoais().nome()
        );
    }

    if let Some(Reg(funcionario_do_mes)) = registros.get_mut(&id_funcionario_do_mes) {
        da_aumento(funcionario_do_mes, 200.0);
    }

    let ids_funcionarios_faltantes = informa_funcionarios_faltantes();
    for id in ids_funcionarios_faltantes.iter() {
        if let Some(Reg(funcionario_faltante)) = registros.get_mut(id) {
            desconta_falta(funcionario_faltante, 50.0);
        }
    }

    if let Some(Reg(funcionario1)) = registros.get_mut(&id_f1) {
        da_aumento(funcionario1, 300.0);
    }

    for pessoa in registros.values() {
        println!(
            "O nome é {}, e o salario é {}",
            pessoa.dados_pessoais().nome(),
            pessoa.dados_funcionais().salario()
        );
    }

    println!("{}", "-".repeat(30));
    let funcionario1 = registros.remove(&id_f1).unwrap();
    println!("Antes: {funcionario1:?}");

    let gerente = promove(funcionario1);

    let tentativa_resgatar_fulano = registros.get(&id_f1);

    println!("Depois (tentativa de resgate): {tentativa_resgatar_fulano:?}");
    println!("Depois Gerente: {gerente:?}");

    registros.insert(id_f1, gerente);

    println!("{}", "-".repeat(30));
    for registro in registros.iter() {
        println!("{:?}", registro);
    }
}

fn arrays_vs_vectors_vs_tuples() {
    let int_array: [i32; 3] = [1, 2, 3]; //STACK

    let segundo_elemento_do_array = int_array[1];

    let mut int_vector: Vec<i32> = vec![1, 2, 3]; //HEAP - MEMORY LEAKS - gerenciamento de memoria e garbage collector em outras linguagens
                                                  // let mut int_vector: Vec<i32> = Vec::new();
                                                  // int_vector.push(1);
                                                  // int_vector.push(2);
                                                  // int_vector.push(3);

    let algum_elemento_do_vector = int_vector.get(2);
    if let Some(elemento) = algum_elemento_do_vector {
        println!("O elemento é {}", elemento);
    }
    println!("O elemento de indice 2 é {algum_elemento_do_vector:?}");

    let uma_tuple: (i32, i32) = (1, 2);
    println!("primeiro da tuple é {}", uma_tuple.0);
    println!("segundo da tuple é {}", uma_tuple.1);

    let outra_tuple: (&str, i32) = ("Hello", 123);

    let uma_pessoa: (i64, &str, u8, &str) = (12312312312, "Fulano", 30, "Rua das Flores");
}
