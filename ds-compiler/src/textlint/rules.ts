import noInvalidControlCharacter from '@textlint-rule/textlint-rule-no-invalid-control-character';
import maxTen from 'textlint-rule-max-ten';
import noDoubleNegativeJa from 'textlint-rule-no-double-negative-ja';
import noDoubledConjunction from 'textlint-rule-no-doubled-conjunction';
import noDoubledConjunctiveParticleGa from 'textlint-rule-no-doubled-conjunctive-particle-ga';
import noDoubledJoshi from 'textlint-rule-no-doubled-joshi';
import noDroppingTheRa from 'textlint-rule-no-dropping-the-ra';
import noHankakuKana from 'textlint-rule-no-hankaku-kana';
import noMixDearuDesumasu from 'textlint-rule-no-mix-dearu-desumasu';
import noNFD from 'textlint-rule-no-nfd';
import sentenceLength from 'textlint-rule-sentence-length';

export const rules = [
    maxTen,
    noDoubleNegativeJa,
    noDoubledConjunction,
    noDoubledConjunctiveParticleGa,
    noDoubledJoshi,
    noDroppingTheRa,
    noHankakuKana,
    noInvalidControlCharacter,
    noMixDearuDesumasu,
    noNFD,
    sentenceLength,
];
