# Maze RS
This project was developed to implement algorithms that solves mazes.

## Implemented algorithms
- A*
- Depth First Search

## How Mazes are read and stored
In currently moment this program only accepts five mocked mazes with default sizes:
- 11x11
- 51x51
- 101x101
- 201x201

The maze is a N*N matrix stored in a .txt file and each character can assume one of this forms:
- 0 => Empty Space
- 1 => A Wall or Barrier
- 2 => The Maze's Entry
- 3 => The Maze's Exit

## Task List
- [x] Read Maze From Txt
- [x] Print Maze on Screen
- [x] Optimize Print
- [x] Uninformed Search Algorithm
- [x] Informed Search Algorithm
- [x] User Interface
- [x] Animate Maze's Path Finding
- [x] Add Faster Path Finding Animation
- [x] Fix Maze Rotation
- [x] User Control The Speed of Maze's Path Finding Animation
- [ ] Create Plugins to Make Code Cleaner
- [ ] Create Random Mazes (Optional)
- [ ] Create more generic way to store and read the mazes (Optional)
- [ ] Implement more search algorithms (Optional)