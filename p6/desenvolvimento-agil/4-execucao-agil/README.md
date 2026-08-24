# 4 Execução Ágil

## Reuniões e Rituais (Cerimônias do Scrum)

As cerimônias fornecem cadência e sincronização para equipes multifuncionais lidarem com a complexidade. Todas são _timeboxed_ (têm duração máxima fixa).

### 1. _Sprint Planning_ (Planejamento da Sprint)

- **Objetivo:** Definir a Meta da _Sprint_ e criar o _Sprint Backlog_ (itens do _Product Backlog_ selecionados + tarefas técnicas).
- **Duração:** Máx. 8 horas para _Sprints_ de 30 dias (proporcionalmente menor para _Sprints_ mais curtas).
- **Dinâmica:** O _Product Owner_ (PO) esclarece o valor de negócio e a prioridade dos itens. A Equipe de Desenvolvimento avalia sua capacidade e puxa os itens, definindo "como" o trabalho será feito.

### 2. _Daily Scrum_ (Reunião Diária)

- **Objetivo:** Sincronizar as atividades das próximas 24h, inspecionar o progresso rumo à Meta da _Sprint_ e identificar impedimentos.
- **Duração:** Máx. 15 minutos (realizada preferencialmente de pé, no mesmo local e horário).
- **Dinâmica (3 perguntas):** O que fiz ontem? O que farei hoje? Há algum impedimento?
- **Armadilhas:** Não deve ser uma reunião de reporte de _status_ para o chefe ou _Scrum Master_, mas sim um alinhamento da equipe para a equipe.

### 3. _Sprint Review_ (Revisão da Sprint)

- **Objetivo:** Inspecionar o incremento funcional construído e colher _feedback_.
- **Duração:** Máx. 4 horas para _Sprints_ de 30 dias.
- **Dinâmica:** A equipe apresenta (frequentemente via demonstração prática) o que foi concluído. O PO adapta o _Product Backlog_ com base no _feedback_ dos _stakeholders_.

### 4. _Sprint Retrospective_ (Retrospectiva da Sprint)

- **Objetivo:** Inspecionar pessoas, interações, processos e ferramentas, buscando melhoria contínua (_Kaizen_).
- **Duração:** Máx. 3 horas para _Sprints_ de 30 dias.
- **Dinâmica:** O que foi bem? O que pode melhorar? A equipe sai com um plano de ação (itens concretos de melhoria) para aplicar na próxima _Sprint_.

## Princípios Lean e Práticas XP na Execução

O Scrum fornece o arcabouço gerencial, enquanto o _Extreme Programming_ (XP) traz práticas sólidas de engenharia de _software_ (aplicadas com mentalidade _Lean_):

- **Eliminação de desperdício:** Não criar código ou documentação desnecessária no momento.
- **TDD (_Test-Driven Development_):** Escrever os testes automatizados antes do código de produção. Força o foco no _design_ limpo e em "o que" o sistema deve fazer antes de "como".
- **Integração Contínua (CI):** Fazer o _merge_ do código no repositório principal várias vezes ao dia e rodar baterias de testes automatizados para evitar o "caos de integração" ao final da _Sprint_.
- **Refatoração Contínua:** Melhorar continuamente a estrutura interna do código (sem alterar sua funcionalidade externa) para manter a base sustentável e livre de "débito técnico".
- **Programação em Pares (_Pair Programming_):** Dois desenvolvedores em uma mesma máquina. Gera revisão de código em tempo real e disseminação de conhecimento rápido.

## Resolução de Impedimentos

É papel vital do _Scrum Master_ trabalhar nos bastidores políticos da organização, usando métricas e construindo alianças, para remover os entraves estruturais e bloqueios que impedem a fluidez tática da Equipe de Desenvolvimento.
