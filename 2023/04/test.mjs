#!/usr/local/bin/node

import fs from 'fs';

const content = fs.readFileSync(process.argv[2]).toString();

const lines = content.split("\n");

const cards = lines
    .filter(line => line.trim())
    .map(line => line.split(':')[1])
    .map(line =>
        line
            .split('|')
            .map(side => side.trim())
            .map(side => side.split(/\s+/g))
    )
    .map((splitline, num) =>
        ({
            num,
            winning: splitline[0].map(x => parseInt(x)),
            found: splitline[1].map(x => parseInt(x))
        })
    );

const result = cards.reduce((acc, val) =>
    acc + get_card_copies(val, cards)
, 0);

console.log(result);

function get_card_copies(card, card_set) {
    let common_nums = card.winning
        .filter(n => card.found.includes(n));
    
    const start_idx = card.num+1;
    const end_idx = card.num + common_nums.length;
        
    const cards_won = card_set.slice(start_idx, end_idx+1);
        
    let count = 1;
    
    for(const card_copy of cards_won)
    {
        let result = get_card_copies(card_copy, card_set)

        count += result;
    }

    return count
}
