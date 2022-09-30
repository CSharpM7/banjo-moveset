# Old Bear, New Tricks

## Nair
Banjo now has an extremely versatile nair! Banjo swings his backpack 360 degrees, hitting in front, behind and then in front again. The bag is the sweetspot, while Banjo's arms are the sourspot. The first hit is the strongest, comparable to a slightly weaker Samus nair. The back hit is noticeably weaker, but gets opponents off of you. For the final hit, the sweetspot pops opponents up which can lead into fair/upsmash, but is harder to land.

Advantages:
- Safer
- Can combo into fair and upsmash
- KOs earlier
- Less landing lag

Disadvantages:
- No more dragdown nair
- Edgeguarding and 2-framing ledge recoveries is much more difficult
- Weaker hitboxes don't lead into much

Specs:
```diff
! Hitboxes on frames 8-12, 15-19,25-29
- Autocancellable on frames 1-6, 42+
+ Landing lag: 8
- Final Frame: 54
```

## Upsmash
Up smash is now Banjo's uptilt. The changes below are the differences between his original uptilt.

Advantages:
- No longer a multihit
- Stronger than the original upsmash
- Has a launch hitbox at the bottom, hitting small characters and burried foes

Disadvantages:
- Kills about 2% later than the original uptilt
- FAF is later, making this move less spammable and less safe on shield

```diff
Specs:
- Total Frames: +7
- FAF: 32 -> 48
- KBG: 110 -> 109
- BKB: 42 -> 50
+ Launch hitbox added. Active in front of Banjo during frames 10-12. Attempts to launch opponents into the main hitbox
```