import { TextlintMessage } from '@textlint/types';
import noInvalidControlCharacter from '@textlint-rule/textlint-rule-no-invalid-control-character';
import * as A from 'fp-ts/lib/Array';
import { pipe } from 'fp-ts/lib/function';
import * as TE from 'fp-ts/lib/TaskEither';
import { TextLintCore } from 'textlint';
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

// eslint-disable-next-line @typescript-eslint/no-var-requires
const { version } = require('../package.json') as { version: string };
console.log(`DramaScript Compiler v${version}`);

type LintTask = TE.TaskEither<string, TextlintMessage[]>;

const runLintTask = (task: LintTask) => task();

const lintText = (text: string): LintTask =>
    pipe(
        TE.tryCatch(() => {
            const rules = [
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
            const core = new TextLintCore();
            core.setupRules(rules);
            return core.lintText(text);
        }, String),
        TE.map((result) => result.messages)
    );

const showLintMessages: (task: LintTask) => LintTask = TE.map(
    A.map((msg) => {
        console.log(`(Ln ${msg.line}, Col ${msg.column}) ${msg.message}`);
        return msg;
    })
);

void pipe(
    lintText(
        'しかし、私は以下のように思ったが、結果は予想どおりだったが、これは禁止されている半角ｶﾀｶﾅ。しかし、同じ接続詞が連続して出現している。'
    ),
    showLintMessages,
    runLintTask
);
