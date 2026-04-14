## Plan: Implementing Compound Statements and Enhanced Builtins in Ghast

TL;DR: セミコロンで複文をサポートし、printビルトインとnum-bigintベースの多倍長整数を追加。単一の木モデルを維持しつつ、複文をSeqとして扱う。

**Steps**
1. AST拡張: Ghast enumにSeq(Vec<Ghast>)を追加。CoreLang enumにもSeq(Vec<CoreLang>)を追加。
2. パーサー修正: ghast_master()を変更してセミコロンを扱う。セミコロンを優先順位1の左結合演算子として定義。
3. 変換修正: convert_into_core()でGhast::SeqをCoreLang::Seqに変換。
4. 評価修正: eval_with_env()でCoreLang::Seqを順次評価。最後の値を返す。
5. 整数型拡張: LiteralにBigIntを追加。ValueにBigIntを追加。num-bigintクレートを依存に追加。
6. ビルトイン拡張: printを追加。stdoutに出力し、Noneを返す。
7. 演算子拡張: セミコロンをoperator.rsに追加。

**Relevant files**
- ghast.rs — AST定義、パーサー
- corelang.rs — CoreLang定義、評価
- operator.rs — 演算子定義
- Cargo.toml — num-bigint依存追加

**Verification**
1. 例のコード "print 5; print (3 + 7)" をパース・評価し、正しく出力されるかテスト。
2. 多倍長整数: 大きな数を扱えるかテスト。
3. 単一式も動作するか確認。

**Decisions**
- 多倍長整数: num-bigintを使用。
- print: stdoutに出力し、Noneを返す。
- セミコロン: 優先順位1、左結合。

**Further Considerations**
1. プロジェクト方針を.github/ai.mdにドキュメント化するか？（ユーザーの提案）
2. LLVM統合は後回し。
