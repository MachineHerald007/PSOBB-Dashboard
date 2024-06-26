import { characters } from "./Characters"
import { bank_09, bank_11, shared_bank } from "./bank"

export const inventory = `[E] Heaven Striker +20 [Berserk] [0/0/0/0|30] 
Vulcan +9 [Charge] [0/0/0/0|60] 
Frozen Shooter +9 [30/0/0/55|30] 
S-RANK NEEDLE ARREST 70 [Arrest] 
S-RANK MECHGUN DEMONS 50 [Demon's] 
S-RANK NEEDLE HELL 70 [Hell] 
S-RANK SHOT HELL 125 [Hell] 
Baranz Launcher +30 [Charge] [0/0/0/100|50] 
Excalibur [Berserk] [0/0/100/0|35] 
[E] Stealth Suit [0/0 | 8/25] [4S] 
[E] Ranger Wall [0/10 | 0/10] 
[E] V101 
[E] V502 
[E] Centurion/Ability 
[E] Adept 
Stealth [5.98/148.98/47.08/0.18] [M|E|P] [Black] 
Dimate x10 
Trimate x10 
Difluid x10 
Trifluid x10 
Sol Atomizer x10 
Moon Atomizer x10 
Meseta: 289725`

export const inventory_09 = `[E] Heaven Striker +20 [Berserk] [0/0/0/0|30] 
Vulcan +9 [Charge] [0/0/0/0|60] 
Frozen Shooter +9 [30/0/0/55|30] 
S-RANK NEEDLE ARREST 70 [Arrest] 
S-RANK MECHGUN DEMONS 50 [Demon's] 
S-RANK NEEDLE HELL 70 [Hell] 
S-RANK SHOT HELL 125 [Hell] 
Baranz Launcher +30 [Charge] [0/0/0/100|50] 
Excalibur [Berserk] [0/0/100/0|35] 
[E] Stealth Suit [0/0 | 8/25] [4S] 
[E] Ranger Wall [0/10 | 0/10] 
[E] V101 
[E] V502 
[E] Centurion/Ability 
[E] Adept 
Stealth [5.98/148.98/47.08/0.18] [M|E|P] [Black] 
Dimate x10 
Trimate x10 
Difluid x10 
Trifluid x10 
Sol Atomizer x10 
Moon Atomizer x10 
Meseta: 289725`

export const inventory_11 = `[E] Heaven Striker +20 [Berserk] [0/0/0/0|20] 
Assault +9 [Charge] [0/0/0/0|60] 
Frozen Shooter +9 [0/0/0/0|20] 
S-RANK NEEDLE Berserk 70 [Berserk] 
S-RANK SHOT DEMONS 125 [Demons] 
Baranz Launcher +30 [Charge] [0/0/0/100|50] 
Excalibur [Berserk] [0/0/100/0|35] 
[E] Stealth Suit [0/0 | 0/25] [4S] 
[E] Ranger Wall [0/10 | 0/10] 
[E] V101 
[E] V502 
[E] Adept 
[E] Adept 
Stealth [5.98/148.98/47.08/0.18] [M|E|P] [Black] 
Dimate x10 
Trimate x10 
Difluid x10 
Trifluid x10 
Sol Atomizer x10 
Moon Atomizer x10 
Meseta: 289725`

export const all_inventory = [
    {
        character: characters[9],
        inventory: inventory_09,
        bank: bank_09
    },
    {
        character: characters[11],
        inventory:inventory_11,
        bank: bank_11
    }
]