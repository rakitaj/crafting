# Lexical Grammar

What is a regular lexical grammer?
- A regular grammer describes a regular language

A grammer can be written as a 4 tuple (N, &Sigma;, P, S)
- N = set of variables
- &Sigma; (sigma) = set of terminal symbols
- P = production rules for terminals and non-terminals
- S = start symbol

What is a strictly regular grammar?
- All of the production rules in P are either
  - Right regular
  - Left regular

#### Right regular grammer ( right linear grammar)
1. B -> a : where B is a terminal in N and a is a terminal in &Sigma;
2. B -> aC : where B and C are non-terminals in N and a is in &Sigma;
3. B -> &epsilon; : where B is in N and &epsilon; is the empty string (aka length 0)

#### Left regular grammer (left linear grammar)

