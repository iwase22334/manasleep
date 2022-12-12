import sys
import json
import requests


def request_audio(text: str, speaker: str):
    host = 'localhost'
    port = 50031

    params = {
            'text': text,
            'speaker': speaker,
        }
    print(params)

    audio_query = requests.post(
        f'http://{host}:{port}/audio_query',
        params=params
    )

    print(audio_query.text)
    print(json.dumps(audio_query.json()))

    response_wav = requests.post(
        f'http://{host}:{port}/synthesis',
        headers={'Content-Type': 'application/json', },
        params=params,
        data=json.dumps(audio_query.json())
    )

    return response_wav


if __name__ == "__main__":
    # {
    #  "name": "MANA+",
    #  "speaker_uuid": "2932eb06-e388-45bf-a6ba-dbc66a48961e",
    #  "styles": [
    #    { "name": "ふくれっつら", "id": 41 },
    #    { "name": "しょんぼり", "id": 42 }
    #  ],
    #  "version": "1.1.0"
    # }

    # sentence = sys.stdin.readline().rstrip('\n')
    speaker = sys.argv[1]
    sentence = sys.argv[2]
    response = request_audio(sentence, speaker)
    with open(f"{speaker}_{sentence}.wav", "wb") as file:
        file.write(response.content)
