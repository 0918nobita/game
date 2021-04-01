import { pipe } from 'fp-ts/lib/function';

import { lintText, runLintTask, showLintMessages } from './textlint';

// eslint-disable-next-line @typescript-eslint/no-var-requires
const { version } = require('../package.json') as { version: string };
console.log(`DramaScript Compiler v${version}`);

void pipe(
    lintText(
        'しかし、私は以下のように思ったが、結果は予想どおりだったが、これは禁止されている半角ｶﾀｶﾅ。しかし、同じ接続詞が連続して出現している。'
    ),
    showLintMessages,
    runLintTask
);
