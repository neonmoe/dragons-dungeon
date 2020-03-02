# Notes

## Timetable
- Saturday: ~~temp graphics, get entities on screen and fighting~~
  - Spent the day sleeping and otherwise AFK, starting sunday instead.
- Sunday: temp graphics, get entities on screen and fighting
  - Started: 11:00
  - Graphics and entities done, fighting moved to monday.
- Monday: little time, clean up / procrastination day
  - Basic combat is done, item bases have been added, most item
    implementations and pickups are wip.
- Tuesday: dungeon generation, should be playable at this point
- Wednesday: content day, items & enemies
- Thursday: polish / catchup
- Friday: _polish_ / catchup
- Saturday: playtesting, bugfixes

## Core ideas
- Only two items: an offensive and a defensive item.
- Health is represented in hearts that consist of 4 units: a normal
  attack does 4 damage, which can then be doubled / halved according
  to effects.
- Stats:
  - Health: stays the same for the whole game?
  - Damage: one heart (4 units), modified by equipment.
- Offensive item ideas:
  - Sword: double attack damage
  - Scythe: instant kill enemies below half-health
  - Hammer: stun enemies for 1 round on hit
  - Dagger: add a poison on enemies, can stack until some maximum
- Defensive item ideas:
  - Shield: halve incoming damage
  - Vampire teeth: get health from attacking
  - Stopwatch: get two moves for every enemy turn
- Enemy ideas:
  - Skeleton: walks around randomly, attacks if player is in an
    adjacent tile.
  - Cobweb: doesn't move, attaches to the player on collision, soaks
    all damage and prevents movement until the attacks kill the
    cobweb.
  - Zombie: acts every other turn, follows and attacks the player if
    within some range and in the same room.
  - Dragon: big boss at the end, has lots of health, attacks via fire
    breathing which is well telegraphed (highlights tiles hit in red).
- Traps: switches and pressure plates can activate traps, which the
  player can use to their advantage, or sometimes, get struck
  by. Should be easy to spot.

## Aesthetic ideas
- Attacks and movement should be done via animating the sprites moving
  from the origin tile to the target tile, programmatically.
- Each sprite should have two variants, which make up a 2 frame
  animation that can be ran slower/faster depending on if we want to
  draw the player's attention. At least for living things.
- Enemies should have at least two sprites: "idle" and "will attack."
  Which would make 4 sprites per enemy. But it'd look cool!
