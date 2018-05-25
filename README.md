# nzsc_single_player_cli
A wrapper for [nzsc_single_player](https://github.com/nzsc-org/nzsc_single_player) that allows you to play against the computer on the command line.

## TL;DR
```bash
git clone https://github.com/nzsc-org/nzsc_single_player_cli.git
cd nzsc_single_player_cli/
cargo run
```

If you already know how to play NZSC, then you're good to go!
Otherwise, continue reading.

## About NZSC
### Introduction
NZSC is a simple hand game that a friend and I invented when we were in elementary school.
It could be described as a more complex rock-paper-scissors (instead of 9 move permutations, there are 784).
It is generally played with 2 players, though it can be played with more.
In this particular implementation, you are one player, and the computer is the other.

### Overview
The object of NZSC is to be the first player to score 5 _points_.

To score points, each player chooses a _move_, and the player with the winning move scores a point. This is repeated until one player has 5 points.

Unlike rock-paper-scissors, each player has a different set of moves to choose from. That is, in rock-paper-scissors, each player has the same set of moves to choose from (`Rock`, `Paper`, and `Scissors`). In NZSC, the set of moves available to a player is determined by their _character_ and their _booster_.

### Rulebook

#### i. Conventions
Important terms will be italicized the first time they appear. Singular "they" will be used as a gender-neutral pronoun.

#### 0. Set up
Each player begins the game with 0 _points_ and 4 _waits_.


#### 1. Character Selection
Each player must simultaneously choose a unique _character_. If they happen to choose the same character, they must repick until they each have a unique character.

Here is a list of what a player can choose for their character:
```
Ninja
Zombie
Samurai
Clown
```

##### 1.1 Base Moves
Each character has 3 _base moves_ associated with it. When a player chooses a character, they _acquire_ (i.e., get the right to play) those moves. Here are the base moves:

Ninja:
```
Kick
Ninja Sword
Nunchucks
```

Zombie:
```
Rampage
Muscle
Zap
```

Samurai:
```
Samurai Sword
Helmet
Smash
```

Clown:
```
Juggling Knives
Acid Spray
Nose
```

##### 1.2 Boosters
Each character has 3 _boosters_ associated with it. Here are the boosters:

Ninja:
```
Shadow
Speedy
No Booster
```

Zombie:
```
Regenerative
Zombie Corps
No Booster
```

Samurai:
```
Atlas
Strong
No Booster
```

Clown:
```
Backwards
Moustachio
No Booster
```

##### 1.3 Headstarts
Except for `Zombie`, each character gets a _headstart_ (free 1 point) against another character.
`Ninja` gets a headstart against `Samurai`.
`Samurai` get a headstart against `Clown`.
`Clown` get a headstart against `Ninja`.
Headstarts are applied immediately after characters are chosen.

#### 2. Booster Selection
Each player must simultaneously select one booster. The set of available boosters that a player can choose from will depend on what character they are (see 1.2).

##### 2.1 Booster Moves
Each booster has 2 _booster moves_ associated with it (except for `No Booster`, which contains no booster moves). When a player selects a booster, they acquire those booster moves. They will now have 5 moves total (3 base moves + 2 booster moves), unless they chose `No Booster`, in which case they will have 3 moves total (3 base moves + 0 booster moves). Here are the booster moves:

Shadow:
```
Shadow Fireball
Shadow Slip
```

Speedy:
```
Run in Circles
Lightning Fast Karate Chop
```

Regenerative:
```
Regenerate
Gravedigger
```

Zombie Corps:
```
Zombie Corps
Apocalypse
```

Atlas:
```
Lightning
Earthquake
```

Strong:
```
Twist
Bend
```

Backwards:
```
Backwards Moustachio
Nose of the Taunted
```

Moustachio:
```
Mustache Mash
Big Hairy Deal
```

No Booster:
```
```

#### 3. Move Selection
Each player will simultaneously play (i.e., select) a move. Depending on what move each player plays, one, none, or both of the players will be awarded a point (see 3.5 for details). This process is called a _round_. Continue playing rounds until one player scores 5 points.

##### 3.1 Destructive Moves
There are 2 moves in the game known as _destructive moves_:
```
Zap
Acid Spray
```
When a destructive move is played, the other player's move is _destroyed_. That is, they lose the right to select that move for the rest of the game.

##### 3.2 Single-use Moves
There are 3 moves in the game known as _single-use moves_:
```
Zap
Regenerate
Acid Spray
```
When a single-use move is played, it destroys itself. That is, it cannot be played for the rest of the game.

##### 3.3 Three-times-in-a-row Rule
It is illegal to play the same move more than three times **in a row**. To clarify, it is perfectly legal to play the same move more than three times, as long as it's not three consecutive times.

##### 3.4 Consequences for Playing an Illegal Move
Playing an illegal move (i.e., playing a destroyed move, playing a move more than three times in a row, or playing a move that the player hasn't acquired) will nullify the outcome of the round, meaning neither player scores a point regardless of what the outcome would have been. The offender will be penalized by deducting a certain amount of waits. If their waits fall below zero, their waits will be set back to zero and their opponents will be awarded 1 point. To determine the amount of waits deducted, apply the **first applicable** clause out of the following:

###### 3.4.1 Destroyed or Nonexistent Move
If the move has been destroyed, or does not exist (i.e., is not one of the 28 moves defined in this rulebook (e.g., `Rock`)), the offender will lose 4 waits.

###### 3.4.2 More Than Three Times In a Row
If the offender has played the move more than 3 times in a row, the offender will lose 3 waits.

###### 3.4.3 Wrong Booster, Right Character
If the move is from the offender's character's other booster (e.g., a `Speedy Ninja` plays `Shadow Slip`, which is from `Ninja`'s other booster: `Shadow`), the offender will lose 2 waits.

###### 3.4.4 Wrong Character
If none of the the above clauses are applied, the offender will lose 3 waits.

##### 3.5 Determining Who Scores a Point
Every move scores a certain number of points against another move. To determine that number, consult the table at [OUTCOME_TABLE.md](https://github.com/nzsc-org/nzsc_single_player_cli/blob/master/OUTCOME_TABLE.md). For each player, find their move on the x-axis, and their opponent's move on the y-axis. Award the player points equal to the number in that table cell. This will be tedious at first, but if you play enough, you will memorize the outcome of every pair of moves, so you won't have to consult the table. There are 784 (28^2) outcomes, so this will take some time to master.

If you find this intimidating, consider this simplified example from rock-paper-scissors:

|  | x: | Rock | Paper | Scissors |
| --- | --- | --- | --- | --- |
| y: | | | | |
| Rock | | 0 | 1 | 0 |
| Paper | | 0 | 0 | 1 |
| Scissors | | 1 | 0 | 0 |

Suppose Player1 chose `Rock` and Player2 chose `Paper`. First, I would calculate Player1's points, by finding their move (`Rock`) on the x-axis and their opponent's move (`Paper`) on the y-axis. The table cell value is `0`. Then, I would calculate Player2's points by finding  their move (`Paper`) on the x-axis and their opponent's move (`Rock`) on the y-axis. The table cell value is `1`. So the outcome for `Rock` vs. `Paper` would be `0` points to `1`.

Congratulations! You now know how to play NZSC!

#### 4. Appendix

##### 4.1 Glossary
 - _point_: each player starts with 0. A player wins when they are the first to reach 5.
 - _wait_: each player starts with 4. They get deducted if you play an illegal move. If you lose too many, your opponent's will be awarded points.
 - _acquire [a move]_: to get the right to play that move
 - _character_: a set of 3 base moves and 2 boosters (3 including `No Booster`)
 - _headstart_: a point awarded to a character against another character at the beginning of the game
 - _booster_: a set of 2 booster moves
 - _base move_: a move acquired when choosing a character
 - _booster move_: a move acquired when choosing a booster
 - _move_: a general term for base move or booster move
 - _destructive move_: a move that destroys the opponent's move
 - _single-use move_: a move that destroys itself
 - _destroy [a player's move]_: revoke that player's right to use that move for the rest of the game
 - _NZSC_: the name of the game. If you haven't already guessed, it is an acronym for "Ninja Zombie Samurai Clown," the characters of the game.
 - _margin_: how much you win by. You can describe this numerically (1, 2, 3, 4, 5) or with NZSC slang (Clinch, Hypnotization, Obliteration, Annihilation, Wipeout).
 - _Wipeout_: 5-0 victory
 - _Annihilation_: 5-1 victory
 - _Obliteration_: 5-2 victory
 - _Hypnotization_: 5-3 victory
 - _Clinch_: 5-4 victory

##### 4.2 FAQ
 - Q: Why are they called "waits"? A: Back in elementary school, my friend and I would impose limits on how long you could take to pick a move. If you wanted extra time to think, you could say "wait" to use one of your four waits, and get a time extension. As the game developed, waits became a sort of currency that could be bartered with and also deducted from as a penalty, but the name stuck, despite the wait's new role in NZSC.
