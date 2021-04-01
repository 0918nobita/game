import { TextlintMessage } from '@textlint/types';
import * as A from 'fp-ts/lib/Array';
import { pipe } from 'fp-ts/lib/function';
import * as TE from 'fp-ts/lib/TaskEither';
import { TextLintCore } from 'textlint';
import maxTen from 'textlint-rule-max-ten';
import noDoubledConjunction from 'textlint-rule-no-doubled-conjunction';
import noDoubledConjunctiveParticleGa from 'textlint-rule-no-doubled-conjunctive-particle-ga';
import noHankakuKana from 'textlint-rule-no-hankaku-kana';

// eslint-disable-next-line @typescript-eslint/no-var-requires
const { version } = require('../package.json') as { version: string };
console.log(`DramaScript Compiler v${version}`);

type LintTask = TE.TaskEither<string, TextlintMessage[]>;

const runLintTask = (task: LintTask) => task();

const lintText = (text: string): LintTask =>
    pipe(
        TE.tryCatch(() => {
            const rules = [maxTen, noDoubledConjunction, noDoubledConjunctiveParticleGa, noHankakuKana];
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
