import {describe, test, it, expect} from "vitest"
import {RollDice} from "../src/Common";

describe('Roll Dice', () => {
    it('should return a number between 1 and 12', () => {
        for(let i = 0; i < 10000; i++){
            let num = RollDice();

            expect(num).toBeGreaterThanOrEqual(1);
            expect(num).toBeLessThanOrEqual(12);
        }
    })
})

