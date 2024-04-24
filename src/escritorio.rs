use std::fmt::Debug;

#[derive(Debug)]
pub struct Beneficio {
    vale_alimentacao: f64,
    fgts: f64,
}

impl Beneficio {
    pub fn new(vale_alimentacao: f64, fgts: f64) -> Self {
        Self {
            vale_alimentacao,
            fgts,
        }
    }

    pub fn vale_alimentacao(&self) -> f64 {
        self.vale_alimentacao
    }

    pub fn fgts(&self) -> f64 {
        self.fgts
    }

    pub fn calcula_beneficios(&self) -> f64 {
        self.fgts + self.vale_alimentacao
    }
}

#[derive(Debug, Clone)]
pub struct DadosPessoais {
    nome: String,
    idade: u8,
}

impl DadosPessoais {
    pub fn new(nome: String, idade: u8) -> Self {
        Self { nome, idade }
    }

    pub fn nome(&self) -> &str {
        self.nome.as_str()
    }

    pub fn idade(&self) -> u8 {
        self.idade
    }
}

#[derive(Debug, Clone)]
pub struct DadosFuncionais {
    salario: f64,
}

impl DadosFuncionais {
    pub fn new(salario: f64) -> Self {
        Self { salario }
    }

    pub fn salario(&self) -> f64 {
        self.salario
    }

    pub fn set_salario(&mut self, valor: f64) {
        self.salario = valor;
    }
}

macro_rules! impl_assalariado_for_funcionario {
    ($dados_funcionais:ident) => {
        fn salario(&self) -> f64 {
            self.$dados_funcionais.salario()
        }

        fn set_salario(&mut self, valor: f64) {
            self.$dados_funcionais.salario = valor;
        }
    };
}

macro_rules! impl_atributos_funcionais_for_funcionario {
    ($dados_pessoais:ident, $dados_funcionais:ident) => {
        fn dados_pessoais(&self) -> &DadosPessoais {
            &self.$dados_pessoais
        }

        fn dados_funcionais(&self) -> &DadosFuncionais {
            &self.$dados_funcionais
        }
    };
}

pub trait Assalariado {
    fn salario(&self) -> f64;
    fn set_salario(&mut self, valor: f64);
}

pub trait AtributosFuncionais {
    fn dados_funcionais(&self) -> &DadosFuncionais;
    fn dados_pessoais(&self) -> &DadosPessoais;
}

#[derive(Debug)]
pub enum Funcionario {
    Reg(FuncionarioRegular),
    Ger(Gerente),
    Dir(Diretor),
}

use Funcionario::*;

#[derive(Debug)]
pub struct FuncionarioRegular {
    dados_pessoais: DadosPessoais,
    dados_funcionais: DadosFuncionais,
    beneficios: Beneficio,
}

impl FuncionarioRegular {
    pub fn new(
        dados_pessoais: DadosPessoais,
        dados_funcionais: DadosFuncionais,
        beneficios: Beneficio,
    ) -> Self {
        let mut result = Self {
            dados_pessoais,
            dados_funcionais,
            beneficios,
        };

        result.dados_funcionais.set_salario(
            result.dados_funcionais.salario() + result.beneficios.calcula_beneficios(),
        );

        result
    }

    pub fn dados_pessoais(&self) -> &DadosPessoais {
        &self.dados_pessoais
    }

    pub fn beneficios(&self) -> &Beneficio {
        &self.beneficios
    }

    pub fn dados_funcionais(&self) -> &DadosFuncionais {
        &self.dados_funcionais
    }
}

impl Assalariado for FuncionarioRegular {
    impl_assalariado_for_funcionario!(dados_funcionais);
}

impl AtributosFuncionais for FuncionarioRegular {
    impl_atributos_funcionais_for_funcionario!(dados_pessoais, dados_funcionais);
}

#[derive(Debug)]
pub struct Gerente {
    dados_pessoais: DadosPessoais,
    dados_funcionais: DadosFuncionais,
    nro_vagas_garagem: u8,
}

impl Gerente {
    pub fn new(
        dados_pessoais: DadosPessoais,
        dados_funcionais: DadosFuncionais,
        nro_vagas_garagem: u8,
    ) -> Self {
        Self {
            dados_pessoais,
            dados_funcionais,
            nro_vagas_garagem,
        }
    }

    pub fn dados_pessoais(&self) -> &DadosPessoais {
        &self.dados_pessoais
    }

    pub fn dados_funcionais(&self) -> &DadosFuncionais {
        &self.dados_funcionais
    }

    pub fn nro_vagas_garagem(&self) -> u8 {
        self.nro_vagas_garagem
    }
}

impl Assalariado for Gerente {
    impl_assalariado_for_funcionario!(dados_funcionais);
}

impl AtributosFuncionais for Gerente {
    impl_atributos_funcionais_for_funcionario!(dados_pessoais, dados_funcionais);
}

//Polimorfismo em Rust usando traits
pub fn desconta_falta<T: Assalariado>(pessoa: &mut T, valor: f64) {
    pessoa.set_salario(pessoa.salario() - valor);
}

pub fn da_aumento<T: Assalariado>(pessoa: &mut T, valor: f64) {
    pessoa.set_salario(pessoa.salario() + valor);
}

// Resolução by Carlos

macro_rules! return_dados_pessoas {
    ($self:expr, $dados:ident) => {
        match $self {
            Reg(pessoa) => pessoa.$dados(),
            Ger(pessoa) => pessoa.$dados(),
            Dir(pessoa) => pessoa.$dados(),
        }
    };
}

impl AtributosFuncionais for Funcionario {
    fn dados_funcionais(&self) -> &DadosFuncionais {
        return_dados_pessoas!(self, dados_funcionais)
    }

    fn dados_pessoais(&self) -> &DadosPessoais {
        return_dados_pessoas!(self, dados_pessoais)
    }
}

pub fn promove(funcionario: Funcionario) -> Funcionario {
    match funcionario {
        Reg(funcionario) => {
            let mut result =
                Gerente::new(funcionario.dados_pessoais, funcionario.dados_funcionais, 2);
            result.set_salario(result.salario() * 2.0);
            Ger(result)
        }
        Ger(funcionario) => {
            let mut result =
                Diretor::new(funcionario.dados_pessoais, funcionario.dados_funcionais, "TI".to_string());
            result.set_salario(result.salario() * 5.0);
            Dir(result)
        },
        _ => {
            println!("WARNING - TENTATIVA DE PROMOVER DIRETOR NÃO SUPORTADA");
            funcionario
        }
    }
}

pub fn rebaixar(funcionario: Funcionario) -> Funcionario {
    match funcionario {
        Ger(funcionario) => {
            let mut result = FuncionarioRegular::new(funcionario.dados_pessoais, funcionario.dados_funcionais, Beneficio::new(
                100.0,
                100.0
            ));
            result.set_salario(result.salario() / 2.00);
            Reg(result)
        }
        Dir(funcionario) => {
            let mut result = Gerente::new(funcionario.dados_pessoais, funcionario.dados_funcionais, 2);
            result.set_salario(result.salario() / 5.00);
            Ger(result)
        },
        _ => {
            println!("WARNING - TENTATIVA DE REBAIXAR FUNCIONARIO NÃO SUPORTADA");
            funcionario
        }
    }
}

#[derive(Debug)]
pub struct Diretor {
    dados_pessoais: DadosPessoais,
    dados_funcionais: DadosFuncionais,
    setor: String,
}

impl Diretor {
    pub fn new(
        dados_pessoais: DadosPessoais,
        dados_funcionais: DadosFuncionais,
        setor: String,
    ) -> Self {
        Self {
            dados_pessoais,
            dados_funcionais,
            setor
        }
    }

    pub fn dados_pessoais(&self) -> &DadosPessoais {
        &self.dados_pessoais
    }

    pub fn dados_funcionais(&self) -> &DadosFuncionais {
        &self.dados_funcionais
    }

    pub fn setor(&self) -> &str {
        self.setor.as_str() // passando as_str para otimizar memória
    }
}

impl Assalariado for Diretor {
    impl_assalariado_for_funcionario!(dados_funcionais);
}

impl AtributosFuncionais for Diretor {
    impl_atributos_funcionais_for_funcionario!(dados_pessoais, dados_funcionais);
}