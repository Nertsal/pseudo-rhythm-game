**Core:**
 - player can do actions at any (or slightly limited) time
 - music beat adjusts to the player's action timing
 - consistent bpm gives bonus to the player
 - higher bpm -> more bonus (but game becomes faster and, therefore, harder as a consequence)

**Core mechanics:**
- Player controls a single character
- Player can do only one [[Action]] per [[Beat]]

**Problems:**
- Player could just move in a safe place (or away from enemies) and accumulate the bonus
- Should enemies, that don't move in beat, affect the music?

**Actions:**
- Move to the neigbour cell
- Attack
- Block

**Enemies:**
- Act rhythmically in one of two ways
- May do multiple [[Action]]s per [[Beat]]

**Enemy beat:**
1. Based on the bpm, i.e. relative to the player's speed.
2. Based on real time, i.e. with their own bpm independent of the player's speed.

**Entity** (player and enemies):
- Can stay in place or move to a cell in [[Von Neumann neighborhood]].
- Can hold up to 2 items (1 for each hand), and use either of them.
- So, entity has 4 actions to choose from on every [[Beat]]: stay, move, use left hand item, use right hand item.

**Movement:**
- Typically entities can move to any [[Cell]] in [[Von Neumann neighborhood]]
- Unless it is occupied by a [[Block]] (but some can be broken).
- If the cell is occupied by another [[Entity]], then [[Contact damage]] is dealt to both entities (which might be imbalanced). If that entity dies, the moving entity takes its place. Otherwise, both entities stay in place.

**Items:**
- Sword - attacks a single cell in [[Moore neighborhood]].
- Shield - blocks damage from 3 cells in front in [[Moore neighborhood]].
- Bow - shoots an arrow (direct [[Projectile]]) towards any cell in [[Moore neighborhood]] with r=5.

**Projectiles:**
- Can be of 2 types:
	1. Direct
		- Travels in a straight line towards the target.
		- Hits any entity that happens to be on the line.
	2. Ballistic
		- Travels over other entities directly to the target.




