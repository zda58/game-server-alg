# Battleship server with heatmap-based agent
<img width="1166" alt="Screenshot 2024-03-09 at 6 06 17 PM" src="https://github.com/axie2335/battleship_algorithm_server/assets/107224274/6648f4a0-f335-4047-b191-f2713aa1a5c4">

A networked battleship game server with an included algorithmic agent

## Server Specifications
The state of the game goes as follows:
1. Server is run and receives the game details.
2. The server prints out its network port, and waits for players to connect.
3. Once both players are connected, the server sends each player the game specifications.
4. The server waits for both players to respond with their ship positions as lists.
5. The server reports the current state of the game to both players.
6. The server sends each player a request for a number of shots equivalent to the number of ships they have left.
7. Each player responds with their shots as a list of coordinates. The number of shots must be
   equal to the request, otherwise the offending player loses.
8. The server receives the shots, and sends each player back a list of the shots that they hit,
   and a list of shots that were damaged on their own board.
9. The server goes back to step 5. This loop continues until one or both of the players have no ships left.
   Then, the server will report to each player their game outcome, stop the game, and begin a new one from step 2.

All messages are sent as JSON. Message formats are specified in specification.md.

<img width="564" alt="image" src="https://github.com/axie2335/battleship_algorithm_server/assets/107224274/b28dc0ba-2870-4d2c-817a-51a8cced5321">

## Build instructions
To run server:
cargo run --bin server

To run player:
cargo run --bin serverplayer
