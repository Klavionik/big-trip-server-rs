SET statement_timeout TO '10s';

WITH data (doc) as (
    VALUES
        ('[
          {
            "id": "884d6798-ae9a-4667-8622-11d79e30f0a4",
            "description": "Venice - for those who value comfort and coziness",
            "name": "Venice",
            "pictures": [
              {
                "src": "https://21.objects.htmlacademy.pro/static/destinations/7.jpg",
                "description": "Venice famous for its crowded street markets with the best street food in Asia"
              },
              {
                "src": "https://21.objects.htmlacademy.pro/static/destinations/2.jpg",
                "description": "Venice middle-eastern paradise"
              },
              {
                "src": "https://21.objects.htmlacademy.pro/static/destinations/13.jpg",
                "description": "Venice a perfect place to stay with a family"
              },
              {
                "src": "https://21.objects.htmlacademy.pro/static/destinations/6.jpg",
                "description": "Venice with an embankment of a mighty river as a centre of attraction"
              },
              {
                "src": "https://21.objects.htmlacademy.pro/static/destinations/13.jpg",
                "description": "Venice full of of cozy canteens where you can try the best coffee in the Middle East"
              }
            ]
          },
          {
            "id": "0d2c6b3c-abc9-45a8-bf10-870a1f7601c5",
            "description": "Helsinki - a true asian pearl",
            "name": "Helsinki",
            "pictures": [
              {
                "src": "https://21.objects.htmlacademy.pro/static/destinations/17.jpg",
                "description": "Helsinki famous for its crowded street markets with the best street food in Asia"
              },
              {
                "src": "https://21.objects.htmlacademy.pro/static/destinations/11.jpg",
                "description": "Helsinki middle-eastern paradise"
              },
              {
                "src": "https://21.objects.htmlacademy.pro/static/destinations/2.jpg",
                "description": "Helsinki famous for its crowded street markets with the best street food in Asia"
              },
              {
                "src": "https://21.objects.htmlacademy.pro/static/destinations/5.jpg",
                "description": "Helsinki a perfect place to stay with a family"
              }
            ]
          },
          {
            "id": "c89f1de2-9713-460d-98fc-07c4be5bb541",
            "description": "Madrid - with crowded streets",
            "name": "Madrid",
            "pictures": []
          },
          {
            "id": "3b855e80-3ad5-4c14-a08b-9d2fc6257765",
            "description": "",
            "name": "Sochi",
            "pictures": []
          },
          {
            "id": "49092cd4-166f-479a-b932-51aa8dbffc83",
            "description": "Amsterdam - a perfect place to stay with a family",
            "name": "Amsterdam",
            "pictures": []
          },
          {
            "id": "6f1def18-5edc-4ee4-9ced-60acf3d42a99",
            "description": "Kioto - a true asian pearl",
            "name": "Kioto",
            "pictures": [
              {
                "src": "https://21.objects.htmlacademy.pro/static/destinations/4.jpg",
                "description": "Kioto with a beautiful old town"
              },
              {
                "src": "https://21.objects.htmlacademy.pro/static/destinations/17.jpg",
                "description": "Kioto famous for its crowded street markets with the best street food in Asia"
              },
              {
                "src": "https://21.objects.htmlacademy.pro/static/destinations/19.jpg",
                "description": "Kioto famous for its crowded street markets with the best street food in Asia"
              }
            ]
          },
          {
            "id": "e73e0bb2-7a8d-4063-93f7-0b66ae7b7dc0",
            "description": "Chamonix - with a beautiful old town",
            "name": "Chamonix",
            "pictures": [
              {
                "src": "https://21.objects.htmlacademy.pro/static/destinations/7.jpg",
                "description": "Chamonix with crowded streets"
              },
              {
                "src": "https://21.objects.htmlacademy.pro/static/destinations/7.jpg",
                "description": "Chamonix with crowded streets"
              },
              {
                "src": "https://21.objects.htmlacademy.pro/static/destinations/16.jpg",
                "description": "Chamonix a true asian pearl"
              },
              {
                "src": "https://21.objects.htmlacademy.pro/static/destinations/6.jpg",
                "description": "Chamonix for those who value comfort and coziness"
              },
              {
                "src": "https://21.objects.htmlacademy.pro/static/destinations/18.jpg",
                "description": "Chamonix in a middle of Europe"
              }
            ]
          },
          {
            "id": "fce93263-5508-4848-9e47-c2c63f166ad1",
            "description": "",
            "name": "Valencia",
            "pictures": []
          },
          {
            "id": "39a30b1d-681b-4ec3-b098-27c9f716b834",
            "description": "Moscow - in a middle of Europe",
            "name": "Moscow",
            "pictures": [
              {
                "src": "https://21.objects.htmlacademy.pro/static/destinations/11.jpg",
                "description": "Moscow in a middle of Europe"
              },
              {
                "src": "https://21.objects.htmlacademy.pro/static/destinations/11.jpg",
                "description": "Moscow is a beautiful city"
              },
              {
                "src": "https://21.objects.htmlacademy.pro/static/destinations/11.jpg",
                "description": "Moscow is a beautiful city"
              },
              {
                "src": "https://21.objects.htmlacademy.pro/static/destinations/20.jpg",
                "description": "Moscow middle-eastern paradise"
              },
              {
                "src": "https://21.objects.htmlacademy.pro/static/destinations/19.jpg",
                "description": "Moscow with an embankment of a mighty river as a centre of attraction"
              }
            ]
          },
          {
            "id": "77d6a554-d376-4ec8-afbb-485e8887434f",
            "description": "Tokio - is a beautiful city",
            "name": "Tokio",
            "pictures": [
              {
                "src": "https://21.objects.htmlacademy.pro/static/destinations/16.jpg",
                "description": "Tokio famous for its crowded street markets with the best street food in Asia"
              },
              {
                "src": "https://21.objects.htmlacademy.pro/static/destinations/18.jpg",
                "description": "Tokio in a middle of Europe"
              }
            ]
          }
        ]
        '::jsonb)
)
INSERT INTO destinations (id, name, description, pictures)
SELECT recordset.*
FROM data
CROSS JOIN LATERAL jsonb_populate_recordset(null::destinations, data.doc) as recordset;
