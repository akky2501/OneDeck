# OneDeck
Game Navigation Language for Playing Cards (WIP)

このプロジェクトは[株式会社なうデータ研究所](https://www.nau.co.jp/)様のインターン成果です．
(インターン期間 8/13 - 8/23，9/5)

## What
- トランプを用いたカードゲームとプレイヤーの行動選択を記述するための言語
- ゲーム進行のナビゲーションシステム

詳細はdocument内の資料を参照

## System Overview
![systemoverview](https://github.com/akky2501/OneDeck/blob/master/document/system_overview.png)

## Example
```
Cards 山札 初期値 Fulldeck-{'JK1 'JK2}
Cards 手札A
Cards 手札B
Cards 手札C
Cards 手札D
Cards 場札
Cards 捨山

Actor ゲームマスター ゲーム51(A, B, C, D, 山札)
Actor A プレイヤー(手札A, 山札, 場札, 捨山, ゲームマスター)
Actor B プレイヤー(手札B, 山札, 場札, 捨山, ゲームマスター)
Actor C プレイヤー(手札C, 山札, 場札, 捨山, ゲームマスター)
Actor D プレイヤー(手札D, 山札, 場札, 捨山, ゲームマスター)

Entry [ゲームマスター]の[準備]を行う。

Role プレイヤー (自分の手札, 山札, 場札, 捨山, ゲームマスター) {
  準備 {
      配布: [山札]から['5]枚を[自分の手札]へ追加する。
  }

  1巡目 {
    When [自分の手札]は[?P]を含む && [?Q]は[場札]に含まれる ->
            交換(?P,?Q): [自分の手札]の[?P]と[場札]の[?Q]を交換する。
  }

  2巡目以降 {
    When [自分の手札]は[?P]を含む && [?Q]は[場札]に含まれる ->
            交換(?P,?Q): [自分の手札]の[?P]と[場札]の[?Q]を交換する。
    取り換え: [自分の手札]と[場札]を交換する。
    破棄: [場札]を[捨山]へ移動する、[山札]から['5]枚を[場札]へ追加する。
    パス: 何もしない。
  }

  2巡目以降で場を流した時 {
    When [自分の手札]は[?P]を含む && [?Q]は[場札]に含まれる ->
            交換(?P,?Q): [自分の手札]の[?P]と[場札]の[?Q]を交換する。
    取り換え: [自分の手札]と[場札]を交換する。
    パス: 何もしない。
  }

  コール選択 {
    コールする: [ゲームマスター]の[結果の評価]を行う。
    コールしない: 何もしない。
  }
}

Role ゲーム51 (A, B, C, D, 山札){
  準備 {
    _: [山札]をシャッフルする、
       [A]の[準備]を行う、[B]の[準備]を行う、[C]の[準備]を行う、[D]の[準備]を行う、
       [自分]の[1巡目]を行う。 
  }

  1巡目 {
    _: [A]の[1巡目]を行う、[B]の[1巡目]を行う、[C]の[1巡目]を行う、[D]の[1巡目]を行う、
       [自分]の[2巡目以降]を行う。
  }

  2巡目以降 {
    _: [A]の[2巡目以降]を行う、
       [A]が[破棄]をしたならば、[[A]の[2巡目以降で場を流した時]を行う]、
       [A]の[コール選択]を行う、
       [B]の[2巡目以降]を行う、
       [B]が[破棄]をしたならば、[[B]の[2巡目以降で場を流した時]を行う]、
       [B]の[コール選択]を行う、
       [C]の[2巡目以降]を行う、
       [C]が[破棄]をしたならば、[[C]の[2巡目以降で場を流した時]を行う]、
       [C]の[コール選択]を行う、
       [D]の[2巡目以降]を行う、
       [D]が[破棄]をしたならば、[[D]の[2巡目以降で場を流した時]を行う]、
       [D]の[コール選択]を行う、
       [自分]の[2巡目以降]を行う。
  }

  結果の評価 { _: 終了する。}
}
```

## TODO
- [x] Lexer / Parser
- [ ] Interpreter
- [ ] Assumption Generator
- [ ] Viewer
