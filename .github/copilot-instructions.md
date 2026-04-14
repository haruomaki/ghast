## Plan: Implementing Compound Statements and Enhanced Builtins in Ghast

TL;DR: セミコロンで複文をサポートし、printビルトインを追加。単一の木モデルを維持しつつ、複文をSeqとして扱う。

**Steps**
1. AST拡張: Ghast enumにSeq(Vec<Ghast>)を追加。CoreLang enumにもSeq(Vec<CoreLang>)を追加。
2. パーサー修正: ghast_master()を変更してセミコロンを扱う。セミコロンを優先順位1の左結合演算子として定義。
3. 変換修正: convert_into_core()でGhast::SeqをCoreLang::Seqに変換。
4. 評価修正: eval_with_env()でCoreLang::Seqを順次評価。最後の値を返す。
5. ビルトイン拡張: printを追加。stdoutに出力し、Noneを返す。
6. 演算子拡張: セミコロンをoperator.rsに追加。

**Relevant files**
- ghast.rs — AST定義、パーサー
- corelang.rs — CoreLang定義、評価
- operator.rs — 演算子定義

**Verification**
1. 例のコード "print 5; print (3 + 7)" をパース・評価し、正しく出力されるかテスト。
2. 単一式も動作するか確認。

**Decisions**
- print: stdoutに出力し、Noneを返す。
- セミコロン: 優先順位1、左結合。
- 数値の表現について検討中。

**Further Considerations**
1. プロジェクト方針を.github/ai.mdにドキュメント化するか？（ユーザーの提案）
2. LLVM統合は後回し。

### 言語設計の新しいパイプライン

1. 文字列
2. トークンツリー（括弧/グループ構造を保持した木）
3. 演算子中間表現（旧 `Ghast`。優先度・結合性を決める直前の一時表現）
4. コアAST＝真 `Ghast`（旧 `CoreLang`。言語の核となる抽象構文木）
6. evalして`Value`を得る

### トークンツリー導入に関する懸念

- トークンツリー＝字句解析を入れるということは、PEG の利点を半分捨てることになる可能性がある
- 特にスペースや改行を活用した柔軟な構文規則は、トークンツリー方式だと扱いにくくなる懸念
- 当初検討していたユーザー定義演算子は、演算子中間表現を完全に内部に押し込んでしまうと拡張性が低くなる

### 追加の考察

- トークンツリーは「マクロ安全性」に優れるが、言語の構文自由度とのトレードオフがある
- 演算子中間表現は内部実装として妥当だが、ユーザ向け拡張のためには公開 API としては慎重に扱うべき
- もし表層構文の柔軟性を重視するなら、トークンツリーと PEG の長所をどう両立させるかを次に検討する必要がある
