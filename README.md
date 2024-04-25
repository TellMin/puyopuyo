# ディレクトリ構造

```rust
src/
├── assets/             # ゲームアセット（画像、音声など）
├── components/         # ゲームのコンポーネント定義
│   ├── block.rs        # ブロック（ぷよ）のコンポーネント
│   ├── grid.rs         # グリッド（ゲームフィールド）のコンポーネント
│   └── player.rs       # プレイヤーのコンポーネント
├── resources/          # ゲーム全体で共有されるリソース
│   ├── game_state.rs   # ゲームの状態（スコア、レベルなど）
│   └── settings.rs     # ゲーム設定
├── systems/            # ゲームのロジックを担うシステム
│   ├── block_movement.rs  # ブロックの動きを制御するシステム
│   ├── collision.rs       # 衝突判定システム
│   ├── spawning.rs        # ブロック生成システム
│   └── scoring.rs         # スコア計算システム
├── plugins/            # ゲームの機能をグループ化したプラグイン
│   ├── gameplay.rs     # ゲームプレイ関連のシステムとリソースをまとめたプラグイン
│   └── ui.rs           # UI関連のシステムとリソースをまとめたプラグイン
├── states/             # ゲームの状態（メニュー、プレイ中、ゲームオーバーなど）
│   ├── menu.rs
│   ├── playing.rs
│   └── game_over.rs
└── main.rs             # ゲームのエントリーポイント
```