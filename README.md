# battleship server with heatmap-based agent
<img width="1166" alt="Screenshot 2024-03-09 at 6 06 17 PM" src="https://github.com/axie2335/battleship_algorithm_server/assets/107224274/6648f4a0-f335-4047-b191-f2713aa1a5c4">
A networked battleship game server with an included algorithmic agent

## Server Specifications
The state of the game goes as follows:
1. Server is run and takes in as io the addresses for the Tcp streams of the two players.
2. Once both players are connected, the server sends each player the game specifications.
3. The server waits for both players to respond with their ship positions as lists.
4. The server reports the current game state to both players.
5. The server sends each player a request for a number of shots equivalent to the number of ships they have left.
6. Each player responds with their shots as a list of coordinates. The number of shots must be
   equal to the request, otherwise the offending player loses.
7. The server receives the shots, and sends each player back a list of the shots that they hit,
   and a list of shots that were damaged on their own board.
8. The server goes back to step 4. This loop continues until one or both of the players have no ships left.
   Then, the server will report to each player their game outcome and stop the game.

All formatted payloads are sent as JSON, and their formats are specified in specification.json.

## Player
An example of a player agent is included in algorithmplayer.
<img width="564" alt="image" src="https://github.com/axie2335/battleship_algorithm_server/assets/107224274/b28dc0ba-2870-4d2c-817a-51a8cced5321">
This agent uses a probabilistic heatmap to find the most likely ship coordinates. Try to beat it with your own.

## Build instructions
To run server:
cargo run --bin server

To run player:
cargo run --bin serverplayer
