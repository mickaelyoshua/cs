# 3 Design Ágil

## Qualidade e Riscos em Projetos Ágeis

O risco (incerteza) diminui à medida que o projeto avança e o _software_ funcional é entregue.

- **Abordagem Tradicional:** Riscos concentrados no final do projeto (testes e integração tardios). Alta chance de falha catastrófica.
- **Abordagem Ágil:** Testes contínuos durante o desenvolvimento. Falhas são descobertas e mitigadas cedo. Alterações de requisitos são tratadas como oportunidades de maximizar valor, não como ameaças ao escopo inicial.

### Planejamento de _Release_ (Liberação)

Previsão macro do que será entregue ao longo de várias _Sprints_. Envolve testadores desde o início para definir critérios de aceite, planejar testes de alto nível e avaliar riscos de implementação.

## Artefatos e Modelagem (UML no contexto Ágil)

Mesmo no ágil, a modelagem visual ajuda a esclarecer requisitos estruturais e de comportamento antes ou durante a codificação.

### Diagramas de Classe (Estrutura)

Representa a estrutura estática do sistema (classes, atributos, métodos e relacionamentos).

- **Relacionamentos:**
  - **Herança ("É um"):** Uma classe deriva de outra (ex: _Carro_ herda de _Veículo_).
  - **Associação/Multiplicidade:** Como as instâncias se conectam (ex: 1 para muitos `1..*`).
  - **Composição/Agregação (Todo-parte):** Objetos complexos formados por objetos menores (ex: _Pedido_ contém _Itens_).

### Casos de Uso (Comportamento do Sistema)

Focam em _o que_ o sistema deve fazer sob a perspectiva do usuário, mapeando interações entre Atores e Funcionalidades.

- **Atores:** Quem ou o que interage com o sistema (usuários, outros sistemas). Podem usar herança (ex: _Cliente Cadastrado_ herda de _Visitante_).
- **Relacionamentos entre Casos de Uso:**
  - `<<include>>`: Inclusão obrigatória de outro fluxo (dependência forte).
  - `<<extend>>`: Inclusão opcional ou comportamento alternativo sob certas condições.

### Caso de Uso Descritivo

Documento textual detalhando o Caso de Uso visual.

- Deve ser focado na visão do usuário (sem jargões técnicos de banco de dados ou código).
- **Estrutura básica:** Atores, Pré-condições, Fluxo Principal (caminho feliz), Fluxos Alternativos/Exceções, e Pós-condições (garantia de sucesso).

### Diagrama de Sequência (Interação Temporal)

Modela a troca de mensagens entre os objetos do sistema ao longo do tempo para realizar uma funcionalidade.

- **Linha de vida (_Lifeline_):** O tempo de existência do objeto durante a interação.
- **Especificação de execução (Ativação):** O período em que o objeto está executando um processo.
- Mostra a chamada de métodos (mensagens assíncronas/síncronas) e mensagens de retorno.
