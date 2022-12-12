
## 音声生成

### 話者取得

ps1 python3 ./script/request-speaker.py | jq . >> script/request-speaker-response.txt

### リクエスト生成

for word in $(cat ../wordlist/v2/japanese-word.txt | shuf | head -n 256); do ps1 python3.exe ../script/01-request-audio-query-generate.py 43 $word | tee ../wip/43_$word.json ; done

### ボイス生成

for file in $(find ../wip/* -type f -name '*.json') ; do ps1 python3.exe 02-request-audio.py 43 ${file%.*} $file ; done

## 参考
- https://ranking.net/shopping/food
- https://www.kenpakusha.co.jp/pdf/pdf_ee7.0_ryouri_ichiran.pdf
- https://www.mext.go.jp/component/b_menu/shingi/toushin/__icsFiles/afieldfile/2010/11/16/1299052_3_1.pdf
