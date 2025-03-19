SET statement_timeout TO '10s';

WITH data (doc) as (
    VALUES
        ('[
          {
            "type": "taxi",
            "offers": [
              {
                "id": "90211cf0-0057-4522-ac1e-2f5bbb83a79d",
                "title": "Upgrade to a business class",
                "price": 158
              },
              {
                "id": "5ee94432-e635-4cbc-a9b2-9c67cbcae3fd",
                "title": "Choose the radio station",
                "price": 70
              },
              {
                "id": "5b31c7c4-b466-4f48-8313-8a79ba3593b2",
                "title": "Choose temperature",
                "price": 123
              },
              {
                "id": "ef4652a4-6032-470b-814b-12049c3abc81",
                "title": "Drive quickly, I''m in a hurry",
                "price": 166
              },
              {
                "id": "d8476a8c-53b2-4160-8d57-f4c269154dca",
                "title": "Drive slowly",
                "price": 34
              }
            ]
          },
          {
            "type": "bus",
            "offers": [
              {
                "id": "45c7357d-38d4-40b4-b4d3-490dca7a2361",
                "title": "Infotainment system",
                "price": 199
              },
              {
                "id": "3dbdf586-abb9-46be-9929-75839e782b7d",
                "title": "Order meal",
                "price": 81
              },
              {
                "id": "ebfd5f9d-743c-4482-b8d0-ab1cab6f85f5",
                "title": "Choose seats",
                "price": 45
              }
            ]
          },
          {
            "type": "train",
            "offers": [
              {
                "id": "393236e2-4fc8-4141-8215-88babc500175",
                "title": "Book a taxi at the arrival point",
                "price": 181
              },
              {
                "id": "527de252-8904-40c2-8eec-73e3290bb9d5",
                "title": "Order a breakfast",
                "price": 152
              },
              {
                "id": "6bac36b7-c621-48bb-9d6d-dfef696abb67",
                "title": "Wake up at a certain time",
                "price": 46
              }
            ]
          },
          {
            "type": "flight",
            "offers": [
              {
                "id": "1c5fd9f1-b4fd-4551-a422-160ce0993ffd",
                "title": "Choose meal",
                "price": 155
              },
              {
                "id": "51632c7f-a836-40f5-8e08-1c21506a6cd1",
                "title": "Choose seats",
                "price": 135
              },
              {
                "id": "57cbaa82-fa2b-41a6-9257-f8cd922c4fb4",
                "title": "Upgrade to comfort class",
                "price": 141
              },
              {
                "id": "585bf015-cb12-462a-837b-119ef1c248b0",
                "title": "Upgrade to business class",
                "price": 103
              },
              {
                "id": "122073a2-d072-45e2-b41e-2e41e54e4240",
                "title": "Add luggage",
                "price": 42
              },
              {
                "id": "e813f310-4797-48d1-829f-513dfb98c69c",
                "title": "Business lounge",
                "price": 58
              }
            ]
          },
          {
            "type": "check-in",
            "offers": [
              {
                "id": "c2712695-b4d6-4689-a7ea-6dbceaabe2e0",
                "title": "Choose the time of check-in",
                "price": 197
              },
              {
                "id": "4a7d1c92-b468-4130-b509-0d870df6c94c",
                "title": "Choose the time of check-out",
                "price": 193
              },
              {
                "id": "8d76105e-ab8f-42c6-9044-f3a65721f682",
                "title": "Add breakfast",
                "price": 138
              },
              {
                "id": "94fd2c6d-4a3b-46bd-a421-ceb5fbc5b457",
                "title": "Laundry",
                "price": 82
              },
              {
                "id": "56995c9d-9ad8-4edd-ab4a-91b616e6f8fb",
                "title": "Order a meal from the restaurant",
                "price": 148
              }
            ]
          },
          {
            "type": "sightseeing",
            "offers": []
          },
          {
            "type": "ship",
            "offers": [
              {
                "id": "692fc222-cead-44e9-97bb-f97096b251c9",
                "title": "Choose meal",
                "price": 99
              },
              {
                "id": "e66a379d-4379-4e24-afd0-ccb854fd6319",
                "title": "Choose seats",
                "price": 170
              },
              {
                "id": "c9ba2339-f5e6-4932-9935-48d237a47683",
                "title": "Upgrade to comfort class",
                "price": 107
              },
              {
                "id": "66c80a33-6a11-4702-b3ed-7046d921343d",
                "title": "Upgrade to business class",
                "price": 33
              },
              {
                "id": "56971724-b07e-46ff-8166-51cc89b3701f",
                "title": "Add luggage",
                "price": 180
              },
              {
                "id": "e11910c7-c455-4168-ad1c-a4a8b167102f",
                "title": "Business lounge",
                "price": 171
              }
            ]
          },
          {
            "type": "drive",
            "offers": [
              {
                "id": "4e390177-9220-493a-91a7-773a2f3107a6",
                "title": "With automatic transmission",
                "price": 40
              },
              {
                "id": "f2162434-cce1-4bd4-b367-78d2e42a46b0",
                "title": "With air conditioning",
                "price": 65
              }
            ]
          },
          {
            "type": "restaurant",
            "offers": [
              {
                "id": "6e19d385-0bc8-45c4-bb61-e7f91d9e0947",
                "title": "Choose live music",
                "price": 76
              },
              {
                "id": "12c1cb30-fb82-406c-9f5c-e6640b783afe",
                "title": "Choose VIP area",
                "price": 82
              }
            ]
          }
        ]'::JSONB)
)
INSERT INTO activities (type, offers)
SELECT recordset.*
FROM data
CROSS JOIN LATERAL jsonb_populate_recordset(null::activities, data.doc) as recordset;
