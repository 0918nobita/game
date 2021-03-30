const { TextLintEngine } = require('textlint');

const engine = new TextLintEngine({ rules: ['no-hankaku-kana'] });

(async () => {
    const results = await engine.executeOnText('こんにちは。\n半角ｶﾀｶﾅ');

    for (const result of results)
        for (const msg of result.messages)
            console.log(`(Ln ${msg.line}, Col ${msg.column}) ${msg.message}`);
})();
