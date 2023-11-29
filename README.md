# Messiah
OZOの出勤・退勤を行うツール  
みんなのメシア

# 環境設定
- git clone [url]
- make build-[OS]
- mv <ビルド物のパス。./target以下にビルド物ができる> <コマンドのパスが通っているディレクトリ。Macなら/usr/local/bin>
- messiah -h

# コマンド 
## execute
OZOの出勤・退勤を行う。
```
messiah execute -h [attendance, leaving]
```
- -t, --type
    - 出勤(attendance) or 退勤(leaving)。必須。
- -H, --holiday
    - 土日・祝日の実行をスキップするか？。任意(デフォルトはtrue)。
- -d, --display
    - ヘッドレスモード。任意(デフォルトはtrue)。
## download
[内閣府が公開している](https://www8.cao.go.jp/chosei/shukujitsu/gaiyou.html)祝日csvをダウンロードする。  
ダウンロード先は、configファイルが格納されるディレクトリになる。
```
messiah download
```
- -u, --url
    - ダウンロード先URL。任意(デフォルトは"https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv")。
## show-config
Configのデータを確認する。
```
messiah show-config
```
## set-config
Configのデータを確認する。
```
messiah set-config -u [URL] -i [ID] -p [Password]
```
- -u, --url
    - OZOのURL。任意(省略した場合は設定がスキップされるが、実行する際には必須になる)。
- -i, --id
    - OZOのユーザID。任意(省略した場合は設定がスキップされるが、実行する際には必須になる)。
- -p, --password
    - OZOのパスワード。任意(省略した場合は設定がスキップされるが、実行する際には必須になる)。