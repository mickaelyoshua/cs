# 5 Controle Ágil

## Medidas, Métricas e Indicadores

Para controlar um projeto de forma eficaz, precisamos medir seu desempenho.

- **Medida:** Valor diretamente observável (ex: horas gastas, bugs reportados).
- **Métrica:** Derivada de medidas para analisar tendências (ex: _bugs_ por linha de código, custo de aquisição de cliente).
- **Indicador (KPI):** Métrica escolhida especificamente por refletir o sucesso de um objetivo de negócio.

### Dimensões de Desempenho

- **Eficiência:** Fazer corretamente (uso otimizado de recursos/tempo).
- **Eficácia:** Fazer o que deve ser feito (atingir a meta definida).
- **Efetividade:** Impacto real no mundo (mudança gerada, valor pro cliente).

## Metas SMART

Acrônimo essencial para criar metas e indicadores válidos:

- **S**pecific (Específico): Claro e direto.
- **M**easurable (Mensurável): Critérios quantitativos.
- **A**ttainable (Atingível): Realista, dentro da capacidade da equipe.
- **R**elevant (Relevante): Importante para o negócio.
- **T**ime-bound (Temporal): Possui prazo (ex: final da _Sprint_).

## Velocidade (_Velocity_)

Principal métrica de previsibilidade no Scrum. Representa a quantidade de "trabalho concluído" (ex: _Story Points_) entregue em uma _Sprint_.

- Apenas histórias **100% concluídas** (que cumprem o _Definition of Done_) são contabilizadas. Trabalho pela metade não conta.
- Usada para calcular quantas _Sprints_ serão necessárias para consumir o restante do _Product Backlog_.

## Gestão Visual e Gráficos

- **Burndown Chart:** Gráfico de queima. Mostra o **trabalho restante** ao longo do tempo. A linha deve descer em direção a zero. Ponto fraco: não deixa claro se o escopo total do projeto aumentou.
- **Burnup Chart:** Possui duas linhas (trabalho concluído subindo e escopo total no topo). Vantagem: evidencia claramente quando o escopo aumenta no meio do projeto.
- **Fluxo Cumulativo:** Mostra a quantidade de tarefas em cada coluna do _Kanban_ ao longo do tempo, ajudando a identificar gargalos (áreas onde as tarefas estão acumulando).

## Scrumban (Scrum + Kanban)

União de cerimônias do Scrum com fluxo visual do Kanban.

- **WIP Limits (_Work in Progress_):** Limita a quantidade de itens permitidos na coluna "Em andamento". Força a equipe a terminar o que começou antes de puxar algo novo ("Pare de começar, comece a terminar").

## Ferramentas Comuns

- **Trello:** Abordagem simples, flexível, uso de quadros visuais para qualquer fluxo de trabalho.
- **Jira:** Robusto, voltado ao desenvolvimento de _software_. Gera relatórios como _Burndown_ nativamente e lida bem com rastreio de _bugs_ (Issues) e integrações.
- **Git / GitHub:** Controle de versão distribuído, base para as práticas de engenharia do _Extreme Programming_ (XP) como _Continuous Integration_ (CI).
