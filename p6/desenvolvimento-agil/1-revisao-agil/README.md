# 1 Revisão Ágil

## Manifesto Ágil

Foco em adaptabilidade, entrega contínua de valor e resposta rápida a mudanças em ambientes incertos, opondo-se ao modelo sequencial tradicional (Cascata/PMBOK). Exige mentalidade ágil (cultura) mais do que apenas processos.

## Scrum

_Framework_ iterativo para lidar com problemas complexos. Criado por Ken Schwaber e Jeff Sutherland.

### Papéis

- **Product Owner (PO):** Define e prioriza o _Product Backlog_. Única fonte de trabalho para a equipe.
- **Scrum Master (SM):** Líder-servo. Remove impedimentos e garante que o Scrum seja compreendido e aplicado. Não gerencia pessoas ou tarefas.
- **Equipe de Desenvolvimento:** Multifuncional e auto-organizada. Não possui sub-equipes ou cargos fixos. Tamanho ideal: 5 a 9 pessoas.

### Artefatos e Conceitos

- **Product Backlog:** Lista ordenada e dinâmica de tudo que é necessário no produto. Itens (PBIs) são fatias de valor, geralmente escritas como _User Stories_.
- **Definition of Done (DoD):** Acordo claro do que significa um incremento estar "Pronto" e entregável no final da _Sprint_.

### Eventos (Cerimônias)

- **Sprint:** Ciclo fixo (2 a 4 semanas) onde um incremento de software funcional é gerado. Não sofre alteração de escopo durante a execução.
- **Sprint Planning:** Planeja o trabalho da _Sprint_. Duração: máx. 2h por semana de _Sprint_.
- **Daily Scrum:** Reunião diária de alinhamento tático. Duração: máx. 15 minutos (mesmo local e hora). 3 perguntas fixas, sem debates. SM não precisa participar, apenas garante que ocorra.
- **Sprint Review:** Demonstração do incremento concluído para _stakeholders_ para colher _feedback_.
- **Sprint Retrospective:** Inspeção e adaptação da própria equipe (processos, ferramentas, interações).
  - _Nota:_ Review + Retrospective totalizam máx. 2h por semana de _Sprint_.

### Sprint Zero

Não oficial no Scrum original, mas usado para configurar ambiente mínimo, arquitetura base e primeiras histórias. Duração curta (dias até máx. 1 semana). Não é fase de infraestrutura ou contratação.

### Estimativas

Feitas coletivamente pela Equipe de Desenvolvimento (ex: _Planning Poker_). Foco no entendimento compartilhado e mapeamento de complexidade/dependências, não apenas no tempo. _Story Points_ indicam esforço, não valor.

## Extreme Programming (XP)

Focado em qualidade técnica e respostas rápidas a mudanças constantes de requisitos. Iterações curtas (ex: 14 dias).

- **Fases:** Planejamento, Análise, Projeto, Execução (Codificação, TDD, Testes Manuais) e Release.

## Lean

Surgido na manufatura japonesa (Taiichi Ohno).

- **Princípios:** Eliminar desperdícios (tudo que não agrega valor ao cliente), produção puxada (baseada na demanda), entrega rápida, qualidade embutida, _Kaizen_ (melhoria contínua). Pessoas e interações valem mais que as ferramentas.
