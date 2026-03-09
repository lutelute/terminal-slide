# INTERACTIVE DEMO

## Graphs, Animation, Python

HTMLスライドで動的コンテンツを埋め込む

→ キーで次へ

---

## Chart.js — Bar Chart

- CDNから `chart.js` を読み込むだけ
- スライド表示時にグラフを初期化
- データ更新ボタンで動的に変化

```
  Sales  ████████████████████████  85
  Users  ██████████████████        65
  Views  ████████████████████████████████  120
```

---

## Chart.js — Line Chart

- ライブデータのシミュレーション
- スライド表示中にデータが追加されていく

```
  100 ┤
   80 ┤          ╭─╮
   60 ┤    ╭─╮  ╭╯ ╰╮  ╭─
   40 ┤ ╭─╯  ╰──╯    ╰──╯
   20 ┤─╯
    0 ┼────────────────────
      Jan  Feb  Mar  Apr  May
```

---

## Chart.js — Doughnut & Radar

### Doughnut

```
       ╭───────╮
     ╱    35%    ╲
    │   ╭─────╮   │  25%
    │   │     │   │
     ╲  ╰─────╯  ╱  20%
       ╰───────╯
          20%
```

### Radar

```
         Speed
          ╱╲
   Power ╱  ╲ Range
        ╱ ╳╳ ╲
        ╲ ╳╳ ╱
   Cost  ╲  ╱ Weight
          ╲╱
        Comfort
```

---

## CSS Animations

- **BOUNCE** — `animation: bounce 2s infinite`
- **SPIN** — `animation: spin 3s linear infinite`
- **PULSE** — `animation: pulse 2s infinite`

ターミナルではアニメーション不可。HTMLスライドならCSS animationが使える。

---

## Animated Bar Chart (Pure CSS)

```
  Rust       ██████████████████████████████████████  92%
  Python     ██████████████████████████████████      85%
  TypeScript ████████████████████████████████        78%
  Go         ██████████████████████████              65%
  C++        ██████████████████████                  55%
```

スライド表示時にアニメーション開始（HTML版）

---

## Counter Animation

```
  ┌──────────┐  ┌──────────┐  ┌──────────┐
  │  USERS   │  │DOWNLOADS │  │  STARS   │
  │  12,847  │  │  58,392  │  │  3,241   │
  └──────────┘  └──────────┘  └──────────┘
```

JavaScriptでカウントアップアニメーション（HTML版）

---

## Typing Effect

```
> const greeting = "Hello, World!";
> console.log(greeting);
Hello, World!
> // 1行ずつタイプライター風に表示
```

HTML版では1文字ずつ表示されるタイピングエフェクト

---

## Python in Browser (Pyodide)

```python
import math

# ブラウザ内でPython実行
primes = [n for n in range(2, 50)
          if all(n % i != 0 for i in range(2, int(math.sqrt(n))+1))]

print(f"Primes under 50: {primes}")
# => [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]
```

Pyodide で初回ロードに数秒かかる

---

## Canvas Particle Animation

```
    *  .    ·      *    .
  .    *      ·  .    *
     ·    .  *      ·    .
  *      ·      .    *
    .  *    ·      .    ·
```

Canvas API でリアルタイムパーティクル描画（HTML版）

マウスを動かすとパーティクルが追従

---

## 使えるもの一覧

- `Chart.js` — Bar, Line, Doughnut, Radar...
- `D3.js` — 高度なデータ可視化
- `Pyodide` — ブラウザ内でPython実行
- `CSS Animation` — bounce, spin, pulse, typing...
- `Canvas API` — パーティクル、描画
- `Three.js` — 3Dグラフィクス
- `Mermaid.js` — フローチャート、シーケンス図
- `Lottie` — After Effectsアニメーション

CDNから読み込むだけ。何でも使えます。
