window.BENCHMARK_DATA = {
  "lastUpdate": 1737710799889,
  "repoUrl": "https://github.com/hasura/ndc-sqlserver",
  "entries": {
    "Component benchmarks": [
      {
        "commit": {
          "author": {
            "email": "danieljamesharvey@gmail.com",
            "name": "Daniel Harvey",
            "username": "danieljharvey"
          },
          "committer": {
            "email": "danieljamesharvey@gmail.com",
            "name": "Daniel Harvey",
            "username": "danieljharvey"
          },
          "distinct": true,
          "id": "64b715ac5cde7d7dab26995c887e378dad125b5d",
          "message": "Fix where test, always run on all branches for testing",
          "timestamp": "2023-11-16T09:54:25Z",
          "tree_id": "5e60d4d2729788193cde90f98e6efb83e621ee37",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/64b715ac5cde7d7dab26995c887e378dad125b5d"
        },
        "date": 1700136751476,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 114.3461805,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 139.67060815,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 111.68848191242111,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 3.9813619653670287,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.2904167803469672,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 406.3647465,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 498.15272404999996,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 407.825248602058,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 4.346379128394517,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.3222530687234494,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 321.109748,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 362.28434,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 312.8047071000326,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 6.965758558190998,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 1.5623940862453034,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 343.740365,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 384.082285,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 333.30440481064414,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 7.458749691914932,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 1.8034477864222702,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "danieljamesharvey@gmail.com",
            "name": "Daniel Harvey",
            "username": "danieljharvey"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": false,
          "id": "00645ed288efae131dd2cfd1dee2a4ee69fdbb93",
          "message": "Merge pull request #72 from hasura/djh/NDAT-1031/enable-benchmarks\n\nEnable benchmarks",
          "timestamp": "2023-11-16T15:22:15Z",
          "tree_id": "dc08459ec96e4eac17c21613eab52bf90e8e9b3c",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/00645ed288efae131dd2cfd1dee2a4ee69fdbb93"
        },
        "date": 1700149005347,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 156.392092,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 189.67084954999999,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 151.44257179480232,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 6.024377658482194,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.41709384215542017,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 584.432388,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 643.250909,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 561.2541788322952,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 6.019010481585838,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.45062877167041526,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 388.1116115,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 459.7440698999999,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 379.3808562916017,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 10.04657409059115,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 1.9286660069990658,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 410.02155949999997,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 579.7347976999997,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 410.24036337447563,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 10.539013631821376,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 2.331901646935216,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "danieljamesharvey@gmail.com",
            "name": "Daniel Harvey",
            "username": "danieljharvey"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d3c5950849bbb82666ebb73ec74cb8331d14fb2a",
          "message": "Merge pull request #77 from hasura/djh/bump-ndc-sdk\n\nBump ndc-sdk to newest and fix capabilities",
          "timestamp": "2023-11-17T11:24:22Z",
          "tree_id": "db6861401d27717e7c23d14849a1e00d2578ba82",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/d3c5950849bbb82666ebb73ec74cb8331d14fb2a"
        },
        "date": 1700221100851,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 124.621645,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 156.31544159999999,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 122.5403746156818,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 4.643418418436312,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.30878630384927397,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 477.846322,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 504.236416,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 460.154550707578,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 5.552380801487857,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.3737187229198127,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 328.35468,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 369.2202143,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 318.51811574241,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 7.558273720861621,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 1.5927663245176231,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 347.7193935,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 394.25766699999997,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 337.2548829619378,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 8.188339758131292,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 1.8189034325258249,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "danieljamesharvey@gmail.com",
            "name": "Daniel Harvey",
            "username": "danieljharvey"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0ee9f0c3f3a9149b23fad66c30c303ffe4ae2794",
          "message": "Merge pull request #78 from hasura/djh/NDAT-1028/add-relationships-test\n\nchore: add very recursive relationship test",
          "timestamp": "2023-11-20T08:53:48Z",
          "tree_id": "fbe7784b9456a5751e58398b9d6d91ea7d873199",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/0ee9f0c3f3a9149b23fad66c30c303ffe4ae2794"
        },
        "date": 1700471329912,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 117.934866,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 142.6617978,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 114.35058246550271,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 4.409592947862549,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.2914958746542929,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 467.48111,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 499.929861,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 451.4730762653889,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 4.772177779298829,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.3754355791165094,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 324.232755,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 374.171648,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 316.6820422275235,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 7.1282271246356,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 1.5540566903601105,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 350.72548700000004,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 503.6640543499997,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 350.41024423347676,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 8.300045390085813,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 1.9644399127159746,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "danieljamesharvey@gmail.com",
            "name": "Daniel Harvey",
            "username": "danieljharvey"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "966c16ba33e6fb2be1f7c307716303bf5dfe2111",
          "message": "Merge pull request #79 from hasura/djh/NDAT-1027/introspect-aggregate-functions\n\nchore: introspect aggregate functions",
          "timestamp": "2023-11-20T12:25:45Z",
          "tree_id": "e5ae80ff79ddc77f4f89b4b696843fc612de22c4",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/966c16ba33e6fb2be1f7c307716303bf5dfe2111"
        },
        "date": 1700483935843,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 155.359,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 224.4849224,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 153.81625845784697,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 6.054987868028235,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.32528298070717626,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 489.926036,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 523.1356212,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 473.2557845395688,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 6.177219130934532,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.3404310412467736,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 347.603092,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 385.195455,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 334.91603670351225,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 9.085692645661311,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 1.5319927937331266,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 372.132853,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 628.4932547,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 386.9795434242548,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 11.286611554670856,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 2.1002760858852616,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "danieljamesharvey@gmail.com",
            "name": "Daniel Harvey",
            "username": "danieljharvey"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2388a06ad4cc0b3812e8bbb78aa70b2c40c0eb7b",
          "message": "Merge pull request #80 from hasura/djh/arc-state\n\nWrap Configuration and State in Arc",
          "timestamp": "2023-11-20T14:15:28Z",
          "tree_id": "79e056e94e7cadef6cad8a46b65725b479fd0d31",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/2388a06ad4cc0b3812e8bbb78aa70b2c40c0eb7b"
        },
        "date": 1700490658853,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 108.2040075,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 165.06309544999985,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 109.4234007558673,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 3.7927664471929603,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.48296035883588184,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 668.308991,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 745.8614349999999,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 641.6014562092138,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 2.711712936019353,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.5486267236073641,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 409.501642,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 611.9598720000001,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 411.06491697116587,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 7.045474849978973,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 2.4101881663174507,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 439.137026,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 509.29803,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 425.5803924752255,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 6.890512854805422,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 2.5052950822925912,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "danieljamesharvey@gmail.com",
            "name": "Daniel Harvey",
            "username": "danieljharvey"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f52423ef23e2ad1a0017f50a0235172146377d65",
          "message": "Merge pull request #81 from hasura/djh/NDAT-1045/add-ndc-test\n\nadd `ndc-test`",
          "timestamp": "2023-11-21T10:25:02Z",
          "tree_id": "18eefc2199c8d17ed2e35f3a79c3ce3384f8a6f6",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/f52423ef23e2ad1a0017f50a0235172146377d65"
        },
        "date": 1700563694522,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 60.492371,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 77.214733,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 59.683960299240084,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 1.9354298683999502,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.19321748182345733,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 359.358839,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 468.925617,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 366.5529708374161,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.3294297828504114,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.17235129658551968,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 236.25259699999998,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 345.239429,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 236.06857084355434,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 2.8039673507962277,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.30721321607870933,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 247.85403200000002,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 285.79151405,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 243.21186896633705,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 2.430415972028868,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.34884719108791984,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "danieljamesharvey@gmail.com",
            "name": "Daniel Harvey",
            "username": "danieljharvey"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0a5d968be9379f4d4c0e9183bb100bb9160c8bc2",
          "message": "Fix benchmark connection string (#84)",
          "timestamp": "2023-11-22T14:44:15Z",
          "tree_id": "48d17a95facdcf6aace85c9e07224d9b14a3e851",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/0a5d968be9379f4d4c0e9183bb100bb9160c8bc2"
        },
        "date": 1700664401447,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 64.600972,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 84.022075,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 64.2973915760775,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 2.1034965472566967,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.19948746465049227,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 470.69606450000003,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 635.47529485,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 468.69672564783446,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.4867966417136813,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.2206528286252825,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 231.369325,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 317.63855579999984,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 235.1312057231697,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 2.769529801920527,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.3034393959172845,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 248.8581765,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 286.65866324999996,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 243.32813646746126,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 2.6741464520613647,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.33342991306569425,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "danieljamesharvey@gmail.com",
            "name": "Daniel Harvey",
            "username": "danieljharvey"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e63e6a3793255d8282b3ffad853e2e0a0ccaf769",
          "message": "Merge pull request #83 from hasura/djh/fix-variables-return-bytes-go-zoom\n\nReturn bytes directly",
          "timestamp": "2023-11-22T14:56:38Z",
          "tree_id": "4cadaf3ecdc7e8e606b962d2c0c0c4de69d7a3b3",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/e63e6a3793255d8282b3ffad853e2e0a0ccaf769"
        },
        "date": 1700665844384,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 63.280143,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 80.78474109999998,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 62.10171117201383,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 2.4963280435471447,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.19906276322574162,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 454.907824,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 479.811307,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 435.60966468501596,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.354459103537181,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.2083865181298969,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 210.731743,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 238.11594699999998,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 206.09356629701307,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 1.988699070257212,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.1888382341185143,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 209.465563,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 245.379384,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 205.0669840553215,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 3.094039166386608,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.15661193079580144,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "danieljamesharvey@gmail.com",
            "name": "Daniel Harvey",
            "username": "danieljharvey"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "42fdda4e5c6817df29f3ebc5facd160f935a48e5",
          "message": "Merge pull request #85 from hasura/djh/NDAT-1033/native-queries\n\nfeature: native queries",
          "timestamp": "2023-11-28T13:49:33Z",
          "tree_id": "f726c473628fabcef191c9bc86eaf3bd04406884",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/42fdda4e5c6817df29f3ebc5facd160f935a48e5"
        },
        "date": 1701180308811,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 101.238059,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 130.6637416,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 97.37799654717539,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 4.5484368905651,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.33796479631811555,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 584.333589,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 617.2784548,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 554.2585740506395,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 3.1589895481356507,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.29367283528197213,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 258.268818,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 313.0531272,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 255.11480666179898,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 3.1385395900917956,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.2329637843552412,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 259.53135699999996,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 403.41037379999966,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 263.824904739353,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 3.2956439150945585,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.2140979768194124,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "danieljamesharvey@gmail.com",
            "name": "Daniel Harvey",
            "username": "danieljharvey"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1e3a43994dbcfac3cbe155175bbb9dc09d657fc6",
          "message": "chore: bump `rustc`, downgrade DB (#93)\n\n* Bump Rustc, downgrade DB to version that works on OSX\r\n\r\n* Use a volume\r\n\r\n* Fix benchmark docker file",
          "timestamp": "2023-12-05T13:14:57Z",
          "tree_id": "b763f8a50b118719cd8230f8e4d2c432c88674c4",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/1e3a43994dbcfac3cbe155175bbb9dc09d657fc6"
        },
        "date": 1701782235402,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 58.31523,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 76.68076504999999,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 57.810256355553186,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 1.8860887581190156,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.18589321055779923,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 388.123012,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 473.204478,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 386.24647580839417,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.4461868146145775,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.16445089078933384,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 217.025681,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 344.77116099999984,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 221.22131830838745,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 2.4432250038436507,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.19301983721499344,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 216.031789,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 246.900897,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 210.46740724273093,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 2.1328588001293554,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.1437093840191723,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "danieljamesharvey@gmail.com",
            "name": "Daniel Harvey",
            "username": "danieljharvey"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3f5eb2ebcd66a515f1e3133cac95b22cc8f9d05c",
          "message": "Merge pull request #92 from hasura/dependabot/cargo/ndc-client-v0.1.0-rc.12\n\nBump ndc-client from v0.1.0-rc.11 to v0.1.0-rc.12",
          "timestamp": "2023-12-05T13:27:09Z",
          "tree_id": "df4b1f47d4ad18ae616b3bbbf6a647c7520ffc57",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/3f5eb2ebcd66a515f1e3133cac95b22cc8f9d05c"
        },
        "date": 1701783761282,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 64.932512,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 88.74912299999994,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 64.79834592444854,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 2.3328296868640876,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.20312014768733752,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 380.974189,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 410.790511,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 368.9122727689155,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.16969391502073,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.16165194260116064,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 202.91765049999998,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 229.746497,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 197.74264015996783,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 2.8525278235886162,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.1848390481116977,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 207.82842,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 291.0038127,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 210.17963596742223,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 2.9144338021570206,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.16113785954727033,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "danieljamesharvey@gmail.com",
            "name": "Daniel Harvey",
            "username": "danieljharvey"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": false,
          "id": "cc73da1dfea5d46535ca2729279982c6d3acedaf",
          "message": "Merge pull request #94 from hasura/djh/explain-endpoint\n\nAdd `explain` endpoint",
          "timestamp": "2023-12-06T10:26:35Z",
          "tree_id": "03de0ac8e689056a4b8d76e8d3fdbf0d10829901",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/cc73da1dfea5d46535ca2729279982c6d3acedaf"
        },
        "date": 1701859369062,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 65.640111,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 86.132667,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 65.16248169200058,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 2.2805206518330863,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.20806206981224515,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 494.061113,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 538.129431,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 478.0279319899081,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.6160455060059462,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.23327527967424602,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 214.783733,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 329.954846,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 217.94559326802082,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 2.5439610316895767,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.20530858357492413,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 207.12824,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 253.73044794999998,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 205.32863062890272,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 2.777442151054913,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.16779585421991602,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "samir@functional.computer",
            "name": "Samir Talwar",
            "username": "SamirTalwar"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "70bead41b7c3ea29de560131e3268ae286296627",
          "message": "Attempt to fix the benchmarks. (#117)\n\n* Simplify the benchmark SQL Server config.\n\n* If healthchecks fail, do not panic, but report something meaningful.\n\n* Be more rigorous about importing Chinook data.\n\n* Grab a Chinook dataset which is way faster to load.\n\nThis one does multiple inserts at once.\n\n* Use a separate configuration file for benchmarks.\n\n* fix test_configure_is_idempotent by updating the connection string\n\n* format\n\n---------\n\nCo-authored-by: Pranshi <pranshi@hasura.io>",
          "timestamp": "2024-04-29T15:00:36Z",
          "tree_id": "b2e86aeee547d94f93ad5827f4c8ac3e03e6ce78",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/70bead41b7c3ea29de560131e3268ae286296627"
        },
        "date": 1714403599839,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 99.575346,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 132.60786859999996,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 101.03180549304749,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 1.5721646122319584,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.1575445973861774,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 595.831206,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 649.2201394,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 570.7209236397958,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 2.693529042308228,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.15178358433162378,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 214.770981,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 359.35088749999994,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 222.0447711500458,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 2.074180895285167,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.15867968381564415,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 212.335756,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 248.24047079999997,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 210.86389320979876,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 1.7383867338662071,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.1494905037779046,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "samir@functional.computer",
            "name": "Samir Talwar",
            "username": "SamirTalwar"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "61429967fde3fa4e8d8557472fee08be86321558",
          "message": "CI: Disable the Cachix daemon. (#113)\n\n* CI: Disable the Cachix daemon.\n\nThe Cachix action seems to hang randomly on cleanup. Let's disable the\ndaemon and see if it helps.\n\n* Missed a couple.\n\n---------\n\nCo-authored-by: pranshi06 <85474619+pranshi06@users.noreply.github.com>",
          "timestamp": "2024-04-29T15:21:41Z",
          "tree_id": "3e6c56e6c2a5c8c0b06c2329953ce8a445957ddb",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/61429967fde3fa4e8d8557472fee08be86321558"
        },
        "date": 1714404869604,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 97.2659375,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 126.93911,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 97.3481480398705,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 1.741862922080628,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.12825635104178335,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 599.705399,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 855.94935,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 594.3207211415831,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 2.909490622247631,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.14489374776888578,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 205.270202,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 248.2345948,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 203.75823426525452,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 2.70496462235306,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.12739074648783602,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 210.279707,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 247.6226403999999,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 209.98405785190818,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 1.9486038667814114,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.15563739275614527,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "85474619+pranshi06@users.noreply.github.com",
            "name": "pranshi06",
            "username": "pranshi06"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3aa0bd4688b73138c13300ecec208d9f8c9bef95",
          "message": "make uri optional for cli initialize command (#116)\n\n* fix cli update function\n\n* wip: test local release to github - update changelog version\n\n* wip: test local release to github - update changelog version\n\n* revert changes to the github workflow file\n\n* update changelog information\n\n* update dependency versions\n\n* address review comments",
          "timestamp": "2024-04-30T09:37:02Z",
          "tree_id": "0537027d59aa56ef237ca07a2cb9ac8fdab5c02b",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/3aa0bd4688b73138c13300ecec208d9f8c9bef95"
        },
        "date": 1714470618267,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 99.53941950000001,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 127.3556081,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 99.50228884168624,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 1.7811999735963013,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.14543031755991634,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 622.661201,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 845.7896109999999,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 606.769436098145,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 2.9317015876716823,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.14837137642207063,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 213.690284,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 259.439163,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 215.0832152575327,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 1.9490879054317816,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.16388330107784144,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 217.51205199999998,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 263.26273575,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 217.726727866222,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 1.8581913520011994,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.16531693800075334,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "danieljamesharvey@gmail.com",
            "name": "Daniel Harvey",
            "username": "danieljharvey"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e26d6bd1a22540dcf5c5b29460260c2d23ff2657",
          "message": "Don't use PR branch of ndc-sdk-rs (#120)\n\n* Don't use PR branch of ndc-sdk-rs\n\n* Use current head commit from ndc-sdk-rs",
          "timestamp": "2024-05-02T12:18:38Z",
          "tree_id": "6e34235390bee834551bff10a8d67eff7f28256c",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/e26d6bd1a22540dcf5c5b29460260c2d23ff2657"
        },
        "date": 1714653093557,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 99.02642,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 126.92051819999995,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 98.99888943995079,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 1.8305821206129025,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.15233380868560997,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 585.1874605,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 827.2083749999995,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 583.3619570414238,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 2.297155862308955,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.14798944282294166,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 211.0205485,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 239.39864515,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 208.544279710964,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 1.7315335691127132,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.14976988182566256,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 208.01469350000002,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 243.4717679,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 204.84196892096492,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 2.376872249475497,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.13651014633064087,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "85474619+pranshi06@users.noreply.github.com",
            "name": "pranshi06",
            "username": "pranshi06"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "98b85753eb232e6635ee3cd5e7cd244ee7bb7534",
          "message": "create a separate crate for configuration (#121)\n\n* create a separate crate for configuration\n\n* remove version1 directory in the configuration crate\n\n* remove unused dependencies\n\n* try adding aarch linux binary\n\n* attempt to remove configuration dependency on ndc-sdk",
          "timestamp": "2024-05-07T13:00:28Z",
          "tree_id": "7c8983f7b0d2dbd8e13637aac409a2b52267581a",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/98b85753eb232e6635ee3cd5e7cd244ee7bb7534"
        },
        "date": 1715087583017,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 99.315046,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 125.8485808,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 99.2458485578476,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 1.9807515382546796,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.1464636830266326,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 599.164294,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 864.6473918,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 592.354995942432,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 2.6744089424357753,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.14363945934919287,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 212.974544,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 263.6117372,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 214.6210443894141,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 1.9132041456176978,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.1594202473088491,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 210.809178,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 260.77092819999996,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 209.4679589530074,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 2.8124049463240794,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.140554580081205,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "brandon@codedmart.com",
            "name": "Brandon Martin",
            "username": "codedmart"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3bf098ce13c90ba4a7fb7e8e087bf510929afb07",
          "message": "Update docker.nix file to add cmd and healthcheck (#123)\n\nThis PR updates the docker.nix file to add the cmd and healthcheck. This\nis now the same as the postgres docker.nix file.",
          "timestamp": "2024-05-15T06:33:15Z",
          "tree_id": "aadcde2ad7400847d0d54c3b2716b31cf10182bb",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/3bf098ce13c90ba4a7fb7e8e087bf510929afb07"
        },
        "date": 1715755276429,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 57.836828999999994,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 68.83807195,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 57.465562727019666,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.6154743419807502,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.08978391844331972,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 385.404403,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 468.1273357999996,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 379.46265596099425,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.1015273812622581,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.07958263288554608,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 135.571565,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 148.066194,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 132.81706509253522,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.7015941203089255,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07796595515483323,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 130.596695,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 144.28890555,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 129.0566671467022,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.6441806802110079,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.07427031002555831,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "85474619+pranshi06@users.noreply.github.com",
            "name": "pranshi06",
            "username": "pranshi06"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "43e851fff262592a8f037810f02bdfc191565284",
          "message": "add changelog for v0.1.1 (#124)\n\nUpdates the changelog for the v0.1.1 release",
          "timestamp": "2024-05-15T07:15:10Z",
          "tree_id": "cb859e2f3933c75b68afad7834efeb993c7357d9",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/43e851fff262592a8f037810f02bdfc191565284"
        },
        "date": 1715757813805,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 57.563964,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 69.49971090000001,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 57.96291854611724,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.6175816386499022,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.08985008620065382,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 385.906646,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 407.00080635,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 371.26213543358335,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.0758977261183986,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.0775383981335224,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 129.67465800000002,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 144.9639295,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 128.62996667680304,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.666429961427383,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.06805429186392943,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 129.108662,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 144.7317402,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 127.79220463990096,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.6226856927794557,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.06996312821514127,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "85474619+pranshi06@users.noreply.github.com",
            "name": "pranshi06",
            "username": "pranshi06"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e5a874f7a0b8c95cf3330f65b5bb8882e7d5efe8",
          "message": "update package version for v0.1.1 release (#125)\n\nupdate package version for v0.1.1 release",
          "timestamp": "2024-05-15T07:45:57Z",
          "tree_id": "7fca3559cde1953db5ea3abf4de95f1d4856f863",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/e5a874f7a0b8c95cf3330f65b5bb8882e7d5efe8"
        },
        "date": 1715759606574,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 59.077268,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 76.2837041,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 59.98141561113509,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.6422667444850063,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.08944396020509102,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 388.427514,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 405.90188885000003,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 374.06086778254394,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 0.9928172110613787,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.08195407298686457,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 130.84511,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 147.1437626,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 129.1867682789008,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.6163459017040793,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07097743836685334,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 132.215685,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 145.9877443,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 129.90170172997384,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.6647451339518682,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.07699860901778859,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "85474619+pranshi06@users.noreply.github.com",
            "name": "pranshi06",
            "username": "pranshi06"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "638a2b608f7a9c4625de7df35c61c909d2ce16b1",
          "message": "update changelog format (#126)",
          "timestamp": "2024-05-15T08:19:48Z",
          "tree_id": "7305020c713c1fe65a3bf10e39627902702157da",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/638a2b608f7a9c4625de7df35c61c909d2ce16b1"
        },
        "date": 1715761669865,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 56.680829,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 65.6396252,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 56.23378075761078,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.5761090899969759,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.08701253806909016,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 381.6073335,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 444.63796604999993,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 373.6629573680458,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 0.9464215954881183,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.07715688834526578,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 128.24944449999998,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 142.66773339999997,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 126.86241410866214,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.5908817909746489,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.06872687344400831,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 130.4180345,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 143.94490564999998,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 128.49214044497006,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.7076765467559767,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.0755222080381081,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "karthikeyan@hasura.io",
            "name": "Karthikeyan Chinnakonda",
            "username": "codingkarthik"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9f64e42233101099a8d997c656314b8b405b7ebd",
          "message": "Native mutation support - Incremental PR - I (#118)\n\nhttps://hasurahq.atlassian.net/browse/ES-99\n\n<!-- The PR description should answer 2 (maybe 3) important questions:\n-->\n\n### What\n\n<!-- What is this PR trying to accomplish (and why, if it's not\nobvious)? -->\n\nThis PR does the following things:\n\n1. Parses `nativeMutations` from the configuration.json file.\n2. Generates the NDC schema of the native mutations.\n\n### How\n\n<!-- How is it trying to accomplish it (what are the implementation\nsteps)? -->\n\nThe schema of a native mutation is created in the following way:\n\n1. A native mutation creates two object types:\na. Object that corresponds to the `columns` that will capture the output\nof the native query.\nb. Object that will be the response of the native query with\n`affected_rows` and `returning` being the fields of the object.",
          "timestamp": "2024-05-27T07:10:46Z",
          "tree_id": "46d8a1c83cec9da5561a2f439d7a899ed2d5425a",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/9f64e42233101099a8d997c656314b8b405b7ebd"
        },
        "date": 1716794350198,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 59.342819000000006,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 75.94112100000001,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 60.774403064888,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.6710458417258636,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.09597786546387736,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 388.58127349999995,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 416.05708985,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 375.93678266754927,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 0.9952976338882991,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.08281749470462557,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 131.51565,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 145.0013648,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 129.90978536715693,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.6150162229747025,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.070898235309794,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 131.9490755,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 146.27346104999998,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 129.8389788323607,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.7029839672414084,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.07667692824926088,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "karthikeyan@hasura.io",
            "name": "Karthikeyan Chinnakonda",
            "username": "codingkarthik"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2aa79366fd514d3dfc9a40f22d0db46c6766e081",
          "message": "Native mutations SQL stuff (Native mutations Incremental PR - II) (#130)\n\n<!-- The PR description should answer 2 (maybe 3) important questions:\n-->\n\n### What\n\nThis PR is the second incremental PR towards adding native mutations\nsupport. This PR extends the SQL support to accommodate native\nmutations.\n\n\n<!-- What is this PR trying to accomplish (and why, if it's not\nobvious)? -->\n\n### How\n\n<!-- How is it trying to accomplish it (what are the implementation\nsteps)? -->",
          "timestamp": "2024-06-03T10:36:06Z",
          "tree_id": "6babb528ed7aa1f91d23299ee5afa0c4adbf7d37",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/2aa79366fd514d3dfc9a40f22d0db46c6766e081"
        },
        "date": 1717411428395,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 58.919172,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 73.66867019999994,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 59.95876734157952,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.7028667875561538,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.09683305448674377,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 381.8419655,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 399.96022634999997,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 366.94625519999903,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 0.9492416284141427,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.08259046310128487,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 130.9091895,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 145.0473905,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 129.0644152776455,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.7095911022701102,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.0825107112697242,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 129.533467,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 143.97724459999998,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 127.87492623298932,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.6938426625313951,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.07783214979799034,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "85474619+pranshi06@users.noreply.github.com",
            "name": "pranshi06",
            "username": "pranshi06"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7ba2be54ec87e08e3d87cb311b42d0a2548c9e30",
          "message": "SQL Server REAMDE docs (#128)\n\nUpdate Readme docs\n\n---------\n\nCo-authored-by: gneeri <jonathan.weiss@hasura.io>",
          "timestamp": "2024-06-04T13:35:00Z",
          "tree_id": "976847366cd386153c813ef05c026a3f847cef49",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/7ba2be54ec87e08e3d87cb311b42d0a2548c9e30"
        },
        "date": 1717508554201,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 60.851941499999995,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 75.51342219999998,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 62.01512699047554,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.6694726607003716,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.0981171868445854,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 382.1793705,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 402.77223195,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 369.10955220044514,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.0405246314041392,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.08400844098033147,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 129.073797,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 146.0098458,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 128.19457440070718,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.6219286163106403,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07490854810852796,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 131.56752,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 146.2366734,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 129.33717818747445,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.6813865649372701,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.08016907821452557,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "85474619+pranshi06@users.noreply.github.com",
            "name": "pranshi06",
            "username": "pranshi06"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4519946a714253064e8d51ee6ad0191a36493a87",
          "message": "update Readme (#132)",
          "timestamp": "2024-06-04T15:03:25Z",
          "tree_id": "cd4e875c0ca2af8471b44f531183f2d849681152",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/4519946a714253064e8d51ee6ad0191a36493a87"
        },
        "date": 1717513897430,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 59.379062,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 74.7100449,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 60.97649244734066,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.6026955124031446,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.09047210513059367,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 381.19811849999996,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 418.44224375,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 369.6692685166909,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 0.990891770030828,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.08768034792332159,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 128.832195,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 146.57741819999998,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 127.47820398839494,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.7318087921503889,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07285787064820355,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 130.9361445,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 143.97261479999997,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 128.47003961135871,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.7098330725343089,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.07970128069294243,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "karthikeyan@hasura.io",
            "name": "Karthikeyan Chinnakonda",
            "username": "codingkarthik"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "56bbb7d87bf91dd73cddf0f3c9df0486ef274a5d",
          "message": "Native mutations execution incremental PR (Incremental PR - III) (#131)\n\n<!-- The PR description should answer 2 (maybe 3) important questions:\n-->\n\n### What\n\n<!-- What is this PR trying to accomplish (and why, if it's not\nobvious)? -->\n\nThis PR is the third incremental PR that follows\nhttps://github.com/hasura/ndc-sqlserver/pull/118 and\nhttps://github.com/hasura/ndc-sqlserver/pull/130. This PR adds the\nexecution logic of the native mutations. Please note that, tests will be\nadded in a separate PR because this PR as it is crosses 1000 lines diff,\nbut most of these changes are cosmetic changes and the core logic of\nthis PR should be around 500-600 LOC.\n\n\n\n### How\n\n<!-- How is it trying to accomplish it (what are the implementation\nsteps)? -->\n\nThis PR adds the execution logic of the native mutations in the\nfollowing manner:\n\n1. Parse a `MutationRequest` and then convert it into a\n`MutationExecutionPlan`.\n2. The `MutationExecutionPlan` contains a `NativeMutationExecutionPlan`\ncontaining relevant information for a native mutation to be executed.\n3. The native mutation query is run first and then the rows obtained\nfrom the database are converted into a JSON array of objects, where each\nobject represents a single row.\n4. A second query is made per mutation operation, where the JSON from\nstep 3 is provided as a parameter and the fields requested in the NDC\nquery are returned in the appropriate format.",
          "timestamp": "2024-06-11T08:49:51Z",
          "tree_id": "2e34a6196aa9177a82b78e3d3f0cf1beda48d4d6",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/56bbb7d87bf91dd73cddf0f3c9df0486ef274a5d"
        },
        "date": 1718096270241,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 58.228442,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 72.2177148,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 58.97456364417259,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.6127746393097198,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.09298045638048842,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 383.297972,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 406.79359999999997,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 369.121737197696,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.070925614269754,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.08130657413541992,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 129.548757,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 143.3155034,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 128.03987128670408,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.6082410296770036,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07117665511851289,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 129.426521,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 143.4681413,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 127.60632340929801,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.5708709382732451,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.06781642505556713,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "karthikeyan@hasura.io",
            "name": "Karthikeyan Chinnakonda",
            "username": "codingkarthik"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6b8a9f2e939b6b9aeaf414f53e3c89a8ae2ad228",
          "message": "Native mutations tests (Incremental PR - IV) (#129)\n\n<!-- The PR description should answer 2 (maybe 3) important questions:\n-->\n\n### What\n\n<!-- What is this PR trying to accomplish (and why, if it's not\nobvious)? -->\n\nThis PR is the fourth and final incremental PR following\nhttps://github.com/hasura/ndc-sqlserver/pull/131, towards adding native\nmutations support to SQL server. This PR only adds tests for the native\nmutations and doesn't do anything else.\n\n### How\n\n<!-- How is it trying to accomplish it (what are the implementation\nsteps)? -->",
          "timestamp": "2024-06-11T10:24:44Z",
          "tree_id": "460e5040c0264300a15dec6cfeaa55f3adf822db",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/6b8a9f2e939b6b9aeaf414f53e3c89a8ae2ad228"
        },
        "date": 1718101897770,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 59.5414235,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 75.45727009999997,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 61.14973796663285,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.6225670785291015,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.09470364955224803,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 381.816948,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 406.14168650000005,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 368.2287348221565,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 0.9784669836968192,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.08148269284886736,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 131.4896625,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 146.29586289999997,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 129.7668050031827,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.6824039210885644,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07910864907187573,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 128.816477,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 142.777531,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 127.52292841535233,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.6171271821978479,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.06896633963270872,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "karthikeyan@hasura.io",
            "name": "Karthikeyan Chinnakonda",
            "username": "codingkarthik"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "76fc0cff094de948200ed72d4b0e41fec01d3c80",
          "message": "add native mutations documentation (#133)\n\n[Rendered Docs\npage](https://github.com/hasura/ndc-sqlserver/blob/kc/native-mutation-docs/docs/usage/native_mutations.md)",
          "timestamp": "2024-06-12T13:22:01Z",
          "tree_id": "2df711c2fafa376aa2383ceeade8fc1dd7730990",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/76fc0cff094de948200ed72d4b0e41fec01d3c80"
        },
        "date": 1718198876329,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 58.288709,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 68.33131959999999,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 58.016395103190256,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.6060892504340316,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.09063245968775005,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 382.07681,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 404.132744,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 367.9829946849322,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 0.9642620766373966,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.07832587374995631,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 130.22413,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 145.52465575,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 128.46255174908123,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.6953983277312545,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07664932799328913,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 130.249467,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 143.328145,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 128.27060173472026,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.618286946865112,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.07510692422508533,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "85474619+pranshi06@users.noreply.github.com",
            "name": "pranshi06",
            "username": "pranshi06"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "876ac93402edf2178022b47c9ebc321530bb4397",
          "message": "read connection string from environment variable (#122)\n\n<!-- The PR description should answer 2 (maybe 3) important questions:\n-->\n\n### What\n\nThe PR add the ability to use an Env Var to read the MSSQL connection\nstring, instead of hard-coding the value in the config file (now the\nconfig file can contain the name of the env var, which can be provided\nwhile starting the server)\n\nBy default, the CLI will add the env var `CONNECTION_URI` for the\nstring.\n\n<!-- What is this PR trying to accomplish (and why, if it's not\nobvious)? -->\n\n### How\n\n<!-- How is it trying to accomplish it (what are the implementation\nsteps)? -->",
          "timestamp": "2024-06-17T09:41:05Z",
          "tree_id": "e555d71b596fb8f6765c798cfdddf326848def34",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/876ac93402edf2178022b47c9ebc321530bb4397"
        },
        "date": 1718617692980,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 57.3357285,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 70.162649,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 57.48466411274083,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.5781783809892715,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.08684740186923388,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 376.416,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 399.3639826,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 364.12429385210964,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 0.9924574711934042,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.08093344256862166,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 127.996807,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 144.83347185,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 127.19735874024954,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.6490270575926615,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.06948665782679486,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 129.141926,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 143.83136894999998,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 127.52352110323267,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.6181681840456861,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.07213274713229198,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "85474619+pranshi06@users.noreply.github.com",
            "name": "pranshi06",
            "username": "pranshi06"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": false,
          "id": "bc0fd3d126f6c142587e014aa900fc6bc90cd59d",
          "message": "prepare for v0.1.2 (#134)",
          "timestamp": "2024-06-17T10:20:11Z",
          "tree_id": "1b47acb1d48389adbcf6aa293cd9c2d7ba20de8a",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/bc0fd3d126f6c142587e014aa900fc6bc90cd59d"
        },
        "date": 1718619979642,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 61.341873500000005,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 78.82893764999999,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 62.93172051776432,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.6652818597643275,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.09290064198650998,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 433.1595355,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 507.7063123499999,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 421.2277530408942,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.0881708916516573,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.08323989544652192,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 139.73069099999998,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 155.58279819999998,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 137.7254789421781,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.6334223300499957,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07600074873344288,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 141.80341800000002,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 156.10538814999998,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 138.9526823525655,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.6769185148858696,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.08034152452561243,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "85474619+pranshi06@users.noreply.github.com",
            "name": "pranshi06",
            "username": "pranshi06"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "63bfe90e444b90d523819e509d993e6fd0f8bd68",
          "message": "update README (#135)\n\nUpdate README docs to list down the new CLI steps to start the connector\n\n---------\n\nCo-authored-by: gneeri <jonathan.weiss@hasura.io>",
          "timestamp": "2024-06-18T16:05:08Z",
          "tree_id": "a996873dd746e6b926a0932b388b6abab12294bb",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/63bfe90e444b90d523819e509d993e6fd0f8bd68"
        },
        "date": 1718727080668,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 58.870133499999994,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 74.3277109,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 60.16783815402307,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.6621160495586338,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.09803036040304502,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 388.69596,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 450.60363829999994,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 379.50299312714833,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.0557756872844948,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.09336824284018136,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 133.744736,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 146.2209258,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 131.0235597614638,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.7133918094667422,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.08203536484349785,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 134.065048,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 147.98302339999998,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 132.27957806582862,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.7169521846438727,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.08132096228690187,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "85474619+pranshi06@users.noreply.github.com",
            "name": "pranshi06",
            "username": "pranshi06"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "cd6a57c42687ba5f8cfab0ee7c2e2b7e9a4be24d",
          "message": "update SQL Server README (#138)\n\n<!-- The PR description should answer 2 (maybe 3) important questions:\n-->\n\n### What\n\n1. Fix typo\n2. Add extra instructions (for webpage rendering, since it does not show\nthe title of the bash script)\n\n<!-- What is this PR trying to accomplish (and why, if it's not\nobvious)? -->",
          "timestamp": "2024-06-28T08:10:41Z",
          "tree_id": "6c8b51164b4c020bffdb6f6e85da1682903095f2",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/cd6a57c42687ba5f8cfab0ee7c2e2b7e9a4be24d"
        },
        "date": 1719562598196,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 58.9321925,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 71.78814815,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 59.3174699429583,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.5776212161354835,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.0875349970685284,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 390.75379,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 416.2426372,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 376.63899543117196,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 0.9786910455057978,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.08385243079214146,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 134.6190075,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 147.74241750000002,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 131.92586985922352,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.7460067425829209,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.08273345051195362,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 131.772823,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 149.1528662,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 130.55237032285163,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.6536444523645173,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.07445948447670513,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "karthikeyan@hasura.io",
            "name": "Karthikeyan Chinnakonda",
            "username": "codingkarthik"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7f7efa739191469caa912d57de4c3c4c2c163719",
          "message": "Fix mutation tests (#140)\n\n<!-- The PR description should answer 2 (maybe 3) important questions:\n-->\n\n### What\n\n<!-- What is this PR trying to accomplish (and why, if it's not\nobvious)? -->\nThis PR separates the testing configuration with the sample\nconfiguration to enable mutation tests to run without any dependency on\nthe sample configuration.\n\n### How\n\n<!-- How is it trying to accomplish it (what are the implementation\nsteps)? -->",
          "timestamp": "2024-06-28T11:15:24Z",
          "tree_id": "1d258892b9ba02bfc844eedc2fd0539e0ce00863",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/7f7efa739191469caa912d57de4c3c4c2c163719"
        },
        "date": 1719573740424,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 58.602974,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 73.2134113,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 59.62571362377778,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.6485947076907692,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.09487832556757607,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 387.642974,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 459.0807563999999,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 378.4389519406619,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 0.9992861011794503,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.07875379003467288,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 133.03391800000003,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 146.95795329999999,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 131.0269359274627,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.6890981520342621,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07814772698029979,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 132.031967,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 143.47576750000002,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 129.3067690837407,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.6686773867387785,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.07825355091746045,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "karthikeyan@hasura.io",
            "name": "Karthikeyan Chinnakonda",
            "username": "codingkarthik"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5602bb4467c6a71f6d866f88a0dc73239b751c31",
          "message": "Stored procedures schema and introspection (Incremental PR - I) (#142)\n\n<!-- The PR description should answer 2 (maybe 3) important questions:\n-->\n\n### What\n\n<!-- What is this PR trying to accomplish (and why, if it's not\nobvious)? -->\n\nThis PR adds introspection and schema support for stored procedures in\nMSSQL. This PR is the first incremental PR towards adding stored\nprocedures support.\n\n### How\n\n<!-- How is it trying to accomplish it (what are the implementation\nsteps)? -->\n\n### Introspection\n\nA database query is made to get all the metadata about stored procedures\npresent in the database. As it is a stored procedure, there is no return\ntype tied to it, so we leave the `returns` part empty and we expect the\nuser to fill in the appropriate schema.\n\n### Schema\n\nWe do schema generation for a stored procedure, only if it has a\n`returns` associated with it. Otherwise, we won't be able to generate a\ngraphql schema for the stored procedure.",
          "timestamp": "2024-07-02T11:11:29Z",
          "tree_id": "92097eff2d8203261267ef9dda14eb4b4d3853be",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/5602bb4467c6a71f6d866f88a0dc73239b751c31"
        },
        "date": 1719919062657,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 58.004858,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 73.2954257,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 59.56345284827081,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.6565708707532423,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.09263251993367418,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 381.041081,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 400.75441559999996,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 367.2774981595868,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 0.946368506094359,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.07473585592795715,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 131.011716,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 143.30232435,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 128.8421042680264,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.6485986534212032,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07570749565835132,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 130.619052,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 146.87943214999999,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 129.08909747638432,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.7118441761220708,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.07349798839120027,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "karthikeyan@hasura.io",
            "name": "Karthikeyan Chinnakonda",
            "username": "codingkarthik"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5d2f334d510c39f349cf022cd1d2690db8c48341",
          "message": "Stored procedures execution (Incremental PR - II) (#141)\n\n<!-- The PR description should answer 2 (maybe 3) important questions:\n-->\n\n### What\n\n<!-- What is this PR trying to accomplish (and why, if it's not\nobvious)? -->\n\nThis PR is the second incremental PR towards adding stored procedures\nsupport and this PR adds the execution logic of the same. The actual PR\nlogic is not very large but due to the tests, the lines diff is around\n18k.\n\n### How\n\n<!-- How is it trying to accomplish it (what are the implementation\nsteps)? -->\n\nSteps:\n\n1. Identify whether the procedure name in the mutation request is a\nstored procedure.\n2. Fetch the metadata of the stored procedure.\n3. Arguments and field validation, throw error if any required argument\nis not provided in the request.\n4. Create a temporary table, the schema of the temporary table is\ndetermined by the `returns` field of the stored procedure. The temporary\ntable is used to write the results of the stored procedure into.\n5. The SQL query is generated in the following format:\n\n```sql\nINSERT INTO #TempTable (col1, col2) EXEC <stored_procedure_name> @arg1_name = arg_value \n```\n\n6. Make another SQL query to query from the temporary table with the\nfields requested and return the response.",
          "timestamp": "2024-07-10T08:14:26Z",
          "tree_id": "6105c204830ae98271df6697179eb177150f3f5a",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/5d2f334d510c39f349cf022cd1d2690db8c48341"
        },
        "date": 1720599800923,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 59.3324855,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 73.81345465,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 60.74041306901292,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.7216590476575675,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.10039512362253589,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 388.646923,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 407.5612432,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 373.29500626238666,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.022360799925707,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.08669448385974474,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 130.982644,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 149.257579,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 130.18945133838605,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.617250228958909,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07081481850850074,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 132.322084,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 145.5666986,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 130.19305705141494,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.6371150304238427,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.071861440680279,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "karthikeyan@hasura.io",
            "name": "Karthikeyan Chinnakonda",
            "username": "codingkarthik"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "931eccf2027b90ff3505b8a7806b3b8a8e908b03",
          "message": "Update `ndc-sqlserver` README (#143)\n\n<!-- The PR description should answer 2 (maybe 3) important questions:\n-->\n\n### What\n\n<!-- What is this PR trying to accomplish (and why, if it's not\nobvious)? -->\nSame as title.\n\n### How\n\n<!-- How is it trying to accomplish it (what are the implementation\nsteps)? -->\n\n---------\n\nCo-authored-by: Pranshi <pranshi@hasura.io>",
          "timestamp": "2024-07-18T07:27:44Z",
          "tree_id": "1f7b9e8bde44dc8c3c74bd34153d1ef6c8385493",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/931eccf2027b90ff3505b8a7806b3b8a8e908b03"
        },
        "date": 1721288056321,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 58.799807,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 73.84009719999999,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 59.70940082829897,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.6300725846201658,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.09059148846506675,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 384.9902505,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 404.6785527,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 370.3408524754097,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.0277254083455887,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.08206460134124739,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 129.22688349999999,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 144.5074285,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 127.88499006814064,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.6341481422962829,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07023862843275962,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 130.517579,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 146.365768,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 129.13674771415324,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.6573449249442262,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.07387672655357248,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "85474619+pranshi06@users.noreply.github.com",
            "name": "pranshi06",
            "username": "pranshi06"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0a341b551bf35a2a1c542614c616a1896e7be103",
          "message": "update README according to the latest template (#144)\n\n1. Updates the README according to the latest template (CLI v2.1)\n2. Use mssql-tools18 version of sqlcmd",
          "timestamp": "2024-09-10T09:49:28Z",
          "tree_id": "97bacf25a616d779a6eb75e47bf8cfaf34c9ba1c",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/0a341b551bf35a2a1c542614c616a1896e7be103"
        },
        "date": 1725962323308,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 58.666783,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 67.5854072,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 58.10397952717676,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.5666942076468615,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.09024392970600947,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 393.032643,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 452.07729390000003,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 383.8648153941272,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.0420506537864753,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.08284735084930835,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 135.3651205,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 148.2251519,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 132.41171603113173,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.7904426655384782,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.0822761655390467,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 133.67339550000003,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 149.37618974999998,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 131.2991766162195,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.7017179699731457,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.07992762319115952,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "karthikeyan@hasura.io",
            "name": "Karthikeyan Chinnakonda",
            "username": "codingkarthik"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "05d7071b043f7ea53d100eacadf67480273a3815",
          "message": "add stored procedures documentation (#145)\n\n<!-- The PR description should answer 2 (maybe 3) important questions:\n-->\n\n### What\n\n<!-- What is this PR trying to accomplish (and why, if it's not\nobvious)? -->\n\n### How\n\n<!-- How is it trying to accomplish it (what are the implementation\nsteps)? -->",
          "timestamp": "2024-09-10T12:24:23Z",
          "tree_id": "e5d1ee026e287d42506f52bdf41b8791c9bfe31f",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/05d7071b043f7ea53d100eacadf67480273a3815"
        },
        "date": 1725971512284,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 59.0012355,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 73.2197526,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 59.83667301976828,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.6462676798249944,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.09555388502274786,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 387.11704999999995,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 406.3548523,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 373.2826444838587,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 0.9765288404661874,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.08171278003063064,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 134.130052,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 148.28411799999998,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 131.6597529380978,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.7437720900273916,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07985939173650437,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 133.53826600000002,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 147.29817764999999,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 131.06890358619285,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.6690589489280114,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.07620393056383967,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "karthikeyan@hasura.io",
            "name": "Karthikeyan Chinnakonda",
            "username": "codingkarthik"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "42e83801ea6aa237b55ea7132bbe651bece53cbe",
          "message": "prepare release v0.2.0 (#146)\n\n<!-- The PR description should answer 2 (maybe 3) important questions:\n-->\n\n### What\n\n<!-- What is this PR trying to accomplish (and why, if it's not\nobvious)? -->\n\n### How\n\n<!-- How is it trying to accomplish it (what are the implementation\nsteps)? -->",
          "timestamp": "2024-09-10T13:12:37Z",
          "tree_id": "632a9400b0e517edfdb0b5e8500c631ac10429d1",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/42e83801ea6aa237b55ea7132bbe651bece53cbe"
        },
        "date": 1725974334024,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 58.18762,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 69.783965,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 58.28016854642567,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.5678888860815547,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.08577599609884137,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 383.196525,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 401.760735,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 370.20222672056735,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 0.9161886691495624,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.07749358941781297,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 133.27832999999998,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 146.40598355,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 130.54687873780338,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.6779160642494446,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07619431404957744,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 130.28759250000002,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 144.30437255,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 128.3781902108639,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.5890665324607767,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.06921210013157199,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "karthikeyan@hasura.io",
            "name": "Karthikeyan Chinnakonda",
            "username": "codingkarthik"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "368b38fbe0acb089198acb99ba6dc46c89f6c184",
          "message": "Add `v0.2.0` changelog (#147)\n\n<!-- The PR description should answer 2 (maybe 3) important questions:\n-->\n\n### What\n\n<!-- What is this PR trying to accomplish (and why, if it's not\nobvious)? -->\n\n### How\n\n<!-- How is it trying to accomplish it (what are the implementation\nsteps)? -->",
          "timestamp": "2024-09-10T13:38:30Z",
          "tree_id": "2183fd3c6a6e8848e5caaa73b2231856a28ae76f",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/368b38fbe0acb089198acb99ba6dc46c89f6c184"
        },
        "date": 1725975893760,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 58.108198,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 72.5350254,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 58.617919983661174,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.6676710376398987,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.09237276157321574,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 386.988034,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 410.98732529999995,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 373.8121669725457,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.008866473862554,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.080063462956522,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 133.240108,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 148.4100542,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 130.95120807193493,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.6759559788743843,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07533707019654325,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 131.702227,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 146.8073296,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 129.90796889770445,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.6115192040604711,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.07095232148101155,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "karthikeyan@hasura.io",
            "name": "Karthikeyan Chinnakonda",
            "username": "codingkarthik"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6b3073473f0c1a4e0b2cea5fe32d077dad44ec6a",
          "message": "Add a subcommand to introspect stored procedures (#148)\n\n<!-- The PR description should answer 2 (maybe 3) important questions:\n-->\n\n### What\n\n<!-- What is this PR trying to accomplish (and why, if it's not\nobvious)? -->\n\nThis PR adds a subcommand `stored-procedures` to the CLI `update`\ncommand to introspect stored procedures separately. This is because the\nstored procedures introspection doesn't include any return type and a\nstored procedure without any return type is not added to the connector's\nschema. After the introspection, the user needs to manually add the\nreturn types of the stored procedures and this information should be\nretained, that's why the introspection of stored procedures is decoupled\nfrom the introspection of the `tables` , `types` etc.\n\n### How\n\n<!-- How is it trying to accomplish it (what are the implementation\nsteps)? -->",
          "timestamp": "2024-09-16T13:03:09Z",
          "tree_id": "972258dfba53bcf8eca6acb4150a8dd6139dade0",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/6b3073473f0c1a4e0b2cea5fe32d077dad44ec6a"
        },
        "date": 1726492164073,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 57.563221,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 65.95289834999998,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 57.12988549923473,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.5868080658036661,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.08844526462641407,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 383.466517,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 408.1157715,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 371.07857104553943,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 0.9726193911908467,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.07970450130667898,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 132.36925,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 148.32566855,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 130.2507818321803,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.735379051104303,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07717278440208278,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 131.76128749999998,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 147.01042875000002,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 129.34519377072354,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.6481170450099398,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.0748861012403812,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "karthikeyan@hasura.io",
            "name": "Karthikeyan Chinnakonda",
            "username": "codingkarthik"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": false,
          "id": "35dd850de629234553b3338f4c1ab976b20d77b6",
          "message": "Prepare release v0.2.1 (#149)\n\n<!-- The PR description should answer 2 (maybe 3) important questions:\n-->\n\n### What\n\n<!-- What is this PR trying to accomplish (and why, if it's not\nobvious)? -->\n\n### How\n\n<!-- How is it trying to accomplish it (what are the implementation\nsteps)? -->",
          "timestamp": "2024-09-16T13:19:33Z",
          "tree_id": "57dde39e5069096dd43615367fd1c8497c9656cd",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/35dd850de629234553b3338f4c1ab976b20d77b6"
        },
        "date": 1726493182403,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 58.364285,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 66.72319625,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 57.98538585970811,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.5896728445652073,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.09010849042337161,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 392.301346,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 467.7175877000002,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 384.660642107627,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.0895369303132156,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.09074799651529733,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 134.319467,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 149.293951,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 131.70748276162251,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.7252245581453565,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07552014809295082,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 135.601523,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 149.07316319999998,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 132.5807784812147,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.7988059709757636,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.07977322460378272,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "karthikeyan@hasura.io",
            "name": "Karthikeyan Chinnakonda",
            "username": "codingkarthik"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3242aa924714108dbb372a44a0cc4e0dcf6bd173",
          "message": "Update stored_procedures.md (#150)\n\n<!-- The PR description should answer 2 (maybe 3) important questions:\n-->\n\n### What\n\n<!-- What is this PR trying to accomplish (and why, if it's not\nobvious)? -->\n\n### How\n\n<!-- How is it trying to accomplish it (what are the implementation\nsteps)? -->",
          "timestamp": "2024-09-19T16:02:24Z",
          "tree_id": "2cc26710302d8e187d91c9e4be9832915db2fc68",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/3242aa924714108dbb372a44a0cc4e0dcf6bd173"
        },
        "date": 1726762136958,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 55.685554499999995,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 71.15204324999999,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 57.00400779268716,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.5377364092351158,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.07406806035398306,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 395.813242,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 413.5728488,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 381.0372543075746,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 0.8991734927308244,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.07247091048227391,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 133.184894,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 145.280816,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 130.6086989237429,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.6019221738439171,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.06545406945792374,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 132.037214,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 149.2527376,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 130.0819499900439,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.6223556594981687,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.06706233067811682,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "karthikeyan@hasura.io",
            "name": "Karthikeyan Chinnakonda",
            "username": "codingkarthik"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f53e2a0e0a11e712d4f78d05d1b166ed827c4275",
          "message": "Update `ndc-sdk` version to `v1.6.0`  (#151)\n\n<!-- The PR description should answer 2 (maybe 3) important questions:\n-->\n\n### What\n\n<!-- What is this PR trying to accomplish (and why, if it's not\nobvious)? -->\n\n### How\n\n<!-- How is it trying to accomplish it (what are the implementation\nsteps)? -->\n\n---------\n\nCo-authored-by: py <pranshi@hasura.io>\nCo-authored-by: pranshi06 <85474619+pranshi06@users.noreply.github.com>",
          "timestamp": "2024-10-03T13:45:45Z",
          "tree_id": "4c8fa742d4d2dd1010730008a61d6b3b4ef29586",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/f53e2a0e0a11e712d4f78d05d1b166ed827c4275"
        },
        "date": 1727963728169,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 56.585598,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 67.97713699999998,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 56.78721826479697,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.5251295881388245,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.07346272269918797,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 401.77931,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 430.3647175,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 387.8719543271102,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.0040159206540125,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.07946329443802412,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 133.0966105,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 148.6518808,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 131.41220169418392,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.5434996754218844,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.06440114459924283,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 132.56102,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 149.14983479999998,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 131.46296376095984,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.5698259040088942,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.06656157728929482,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "85474619+pranshi06@users.noreply.github.com",
            "name": "pranshi06",
            "username": "pranshi06"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c33ff7efdcdef2c337aa40e2e776a4c2fdaca68b",
          "message": "Prepare release v0.2.2 (#152)\n\n<!-- The PR description should answer 2 (maybe 3) important questions:\n-->\n\n### What\n\n<!-- What is this PR trying to accomplish (and why, if it's not\nobvious)? -->\n\n### How\n\n<!-- How is it trying to accomplish it (what are the implementation\nsteps)? -->",
          "timestamp": "2024-10-03T15:34:17Z",
          "tree_id": "e6adfaf59a943dd62e23d0c1ed2e9a022947498d",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/c33ff7efdcdef2c337aa40e2e776a4c2fdaca68b"
        },
        "date": 1727970056053,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 57.296394,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 71.8114846,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 58.1408928622033,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.5443041817147432,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.07540003338709286,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 393.349271,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 414.8361812,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 381.2325730164497,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 0.9197549762818085,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.07025310022893497,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 130.772812,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 148.65794154999998,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 129.42836200211283,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.5308111417063799,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.061855449551023504,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 131.7788765,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 144.7064853,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 129.60856094235857,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.5837279395820474,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.0683155108404117,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "brandon@codedmart.com",
            "name": "Brandon Martin",
            "username": "codedmart"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "205eceea515c0a25fa85b19ceb838d5acecfaa20",
          "message": "Release version 1.0.0 (#154)\n\n<!-- The PR description should answer 2 (maybe 3) important questions:\n-->\n\n### What\nAdded ca certs to docker file to solve azure sql server connection\nerrors. Bumped version to 1.0.0.",
          "timestamp": "2024-12-06T20:25:09Z",
          "tree_id": "f0c9dca0ed3bec7cb7abdec4946c7f08c30acf20",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/205eceea515c0a25fa85b19ceb838d5acecfaa20"
        },
        "date": 1733517301922,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 58.437484,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 72.1933195,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 59.25908007714857,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.581929465392939,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.0914753757930133,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 381.08464,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 415.7944412,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 371.9883482210566,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.0350093034855377,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.07494312926161334,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 130.577181,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 151.8900729,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 128.9336444219411,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.7265135044828526,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07957948088028033,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 129.669439,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 151.1166968,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 128.3469571830229,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.6148184678562245,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.07218211689802677,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "danieljamesharvey@gmail.com",
            "name": "Daniel Harvey",
            "username": "danieljharvey"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f52934410e072ab55e13cb4b2221ccecc360494c",
          "message": "Use `ndc-models` directly to avoid openssl everywhere (#156)\n\n<!-- The PR description should answer 2 (maybe 3) important questions:\n-->\n\n### What\n\nFix a problem in https://github.com/hasura/ndc-sqlserver/pull/155 where\npulling `query-engine-translation` into the configuration crate was\npulling `openssl` in. Turns out we're still pulling `ndc-models` from\nthe `ndc-sdk` here (probably from before `ndc-models` became it's own\ncrate).\n\n### How\n\nUse `ndc-models` directly instead of `ndc-sdk`.",
          "timestamp": "2024-12-11T17:24:58Z",
          "tree_id": "4226a63e76895c7ad1aa7f7c889a0038c98af169",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/f52934410e072ab55e13cb4b2221ccecc360494c"
        },
        "date": 1733938354416,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 57.6802175,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 72.75419105,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 59.11112805082183,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.5946588878270447,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.08912495508219082,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 376.54251350000004,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 411.1249024,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 367.6398790743348,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.004642635724963,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.07799240236655695,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 128.998975,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 154.09529525,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 128.51817180803593,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.6250341964282313,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07108608875994131,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 128.762807,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 152.2388177,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 128.22474717570043,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.6108453460333862,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.07267238740570699,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": false,
          "id": "ef17d07b190f8ca7e4f128571a45da8e3a374cda",
          "message": "Bump cachix/install-nix-action from 24 to 30 (#157)\n\nBumps\n[cachix/install-nix-action](https://github.com/cachix/install-nix-action)\nfrom 24 to 30.\n<details>\n<summary>Release notes</summary>\n<p><em>Sourced from <a\nhref=\"https://github.com/cachix/install-nix-action/releases\">cachix/install-nix-action's\nreleases</a>.</em></p>\n<blockquote>\n<h2>v30</h2>\n<ul>\n<li>Nix: 2.24.7 -&gt; 2.24.9, fixing <a\nhref=\"https://github.com/NixOS/nix/security/advisories/GHSA-6fjr-mq49-mm2c\">GHSA-6fjr-mq49-mm2c</a></li>\n</ul>\n<h2>v29</h2>\n<p>Bumps Nix to <a\nhref=\"https://nix.dev/manual/nix/2.24/release-notes/rl-2.24\">2.24.7</a></p>\n<h2>v28</h2>\n<p>Nix 2.24.6 - <a\nhref=\"https://github.com/NixOS/nix/security/advisories/GHSA-h4vv-h3jq-v493\">https://github.com/NixOS/nix/security/advisories/GHSA-h4vv-h3jq-v493</a></p>\n<h2>v27</h2>\n<h2>What's Changed</h2>\n<ul>\n<li>Enable <code>always-allow-substitutes</code> by default by <a\nhref=\"https://github.com/sandydoo\"><code>@​sandydoo</code></a> in <a\nhref=\"https://redirect.github.com/cachix/install-nix-action/pull/207\">cachix/install-nix-action#207</a></li>\n<li>nix: 2.20.5 -&gt; 2.22.1 by <a\nhref=\"https://github.com/kashw2\"><code>@​kashw2</code></a> in <a\nhref=\"https://redirect.github.com/cachix/install-nix-action/pull/206\">cachix/install-nix-action#206</a></li>\n<li>ci: fix tests by <a\nhref=\"https://github.com/sandydoo\"><code>@​sandydoo</code></a> in <a\nhref=\"https://redirect.github.com/cachix/install-nix-action/pull/208\">cachix/install-nix-action#208</a></li>\n</ul>\n<h2>New Contributors</h2>\n<ul>\n<li><a href=\"https://github.com/kashw2\"><code>@​kashw2</code></a> made\ntheir first contribution in <a\nhref=\"https://redirect.github.com/cachix/install-nix-action/pull/206\">cachix/install-nix-action#206</a></li>\n</ul>\n<p><strong>Full Changelog</strong>: <a\nhref=\"https://github.com/cachix/install-nix-action/compare/v26...V27\">https://github.com/cachix/install-nix-action/compare/v26...V27</a></p>\n<h2>v26</h2>\n<p>Bump to Nix 2.20.5 to address CVE-2024-27297 /\nGHSA-2ffj-w4mj-pg37.</p>\n<h2>install-nix-action-v25</h2>\n<p>Nix 2.19.2</p>\n</blockquote>\n</details>\n<details>\n<summary>Commits</summary>\n<ul>\n<li><a\nhref=\"https://github.com/cachix/install-nix-action/commit/08dcb3a5e62fa31e2da3d490afc4176ef55ecd72\"><code>08dcb3a</code></a>\nMerge pull request <a\nhref=\"https://redirect.github.com/cachix/install-nix-action/issues/217\">#217</a>\nfrom Enzime/bump</li>\n<li><a\nhref=\"https://github.com/cachix/install-nix-action/commit/4204e15198ce9348dda13b658f2b0b45397cf9e7\"><code>4204e15</code></a>\nnix: 2.24.8 -&gt; 2.24.9</li>\n<li><a\nhref=\"https://github.com/cachix/install-nix-action/commit/6a10e2e9fdd7f91408e9f9f95bdc77cf5232b932\"><code>6a10e2e</code></a>\nMerge pull request <a\nhref=\"https://redirect.github.com/cachix/install-nix-action/issues/216\">#216</a>\nfrom Mic92/nix-bump</li>\n<li><a\nhref=\"https://github.com/cachix/install-nix-action/commit/2bb614e91ac4d68577f12de9f9844d1e89c80b8d\"><code>2bb614e</code></a>\nNix: 2.24.7 -&gt; 2.24.8</li>\n<li><a\nhref=\"https://github.com/cachix/install-nix-action/commit/9f70348d77d0422624097c4b7a75563948901306\"><code>9f70348</code></a>\nMerge pull request <a\nhref=\"https://redirect.github.com/cachix/install-nix-action/issues/215\">#215</a>\nfrom Mic92/nix-bump</li>\n<li><a\nhref=\"https://github.com/cachix/install-nix-action/commit/4f91dc2b65dd81b86830c57b0d3bc75eb0502a25\"><code>4f91dc2</code></a>\nNix: 2.24.6 -&gt; 2.24.7</li>\n<li><a\nhref=\"https://github.com/cachix/install-nix-action/commit/3715ab1a11cac9e991980d7b4a28d80c7ebdd8f9\"><code>3715ab1</code></a>\nbump channel</li>\n<li><a\nhref=\"https://github.com/cachix/install-nix-action/commit/1872f1ff9dba0f554710b1ed396fb6b2263ffdfd\"><code>1872f1f</code></a>\nNix: 2.22.1 -&gt; 2.24.6</li>\n<li><a\nhref=\"https://github.com/cachix/install-nix-action/commit/e268b7aa055cc66167f27239fc889bcd1b15683c\"><code>e268b7a</code></a>\nMerge pull request <a\nhref=\"https://redirect.github.com/cachix/install-nix-action/issues/213\">#213</a>\nfrom phaer/patch-1</li>\n<li><a\nhref=\"https://github.com/cachix/install-nix-action/commit/5b8c65d4d79bb2d232054c72252fa78a29c36b8a\"><code>5b8c65d</code></a>\nUpdate README: hardware accel is available now...</li>\n<li>Additional commits viewable in <a\nhref=\"https://github.com/cachix/install-nix-action/compare/v24...v30\">compare\nview</a></li>\n</ul>\n</details>\n<br />\n\n\n[![Dependabot compatibility\nscore](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=cachix/install-nix-action&package-manager=github_actions&previous-version=24&new-version=30)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)\n\nDependabot will resolve any conflicts with this PR as long as you don't\nalter it yourself. You can also trigger a rebase manually by commenting\n`@dependabot rebase`.\n\n[//]: # (dependabot-automerge-start)\n[//]: # (dependabot-automerge-end)\n\n---\n\n<details>\n<summary>Dependabot commands and options</summary>\n<br />\n\nYou can trigger Dependabot actions by commenting on this PR:\n- `@dependabot rebase` will rebase this PR\n- `@dependabot recreate` will recreate this PR, overwriting any edits\nthat have been made to it\n- `@dependabot merge` will merge this PR after your CI passes on it\n- `@dependabot squash and merge` will squash and merge this PR after\nyour CI passes on it\n- `@dependabot cancel merge` will cancel a previously requested merge\nand block automerging\n- `@dependabot reopen` will reopen this PR if it is closed\n- `@dependabot close` will close this PR and stop Dependabot recreating\nit. You can achieve the same result by closing it manually\n- `@dependabot show <dependency name> ignore conditions` will show all\nof the ignore conditions of the specified dependency\n- `@dependabot ignore this major version` will close this PR and stop\nDependabot creating any more for this major version (unless you reopen\nthe PR or upgrade to it yourself)\n- `@dependabot ignore this minor version` will close this PR and stop\nDependabot creating any more for this minor version (unless you reopen\nthe PR or upgrade to it yourself)\n- `@dependabot ignore this dependency` will close this PR and stop\nDependabot creating any more for this dependency (unless you reopen the\nPR or upgrade to it yourself)\n\n\n</details>\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2024-12-11T17:30:29Z",
          "tree_id": "6e8796c90de68cdad2083fb3e7a9fe2f69694858",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/ef17d07b190f8ca7e4f128571a45da8e3a374cda"
        },
        "date": 1733938658889,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 58.539628,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 71.50815185,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 58.6393758807307,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.6263677568040507,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.09516827627888248,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 391.981001,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 471.45374675,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 384.1669610885548,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.0918897409113129,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.08932565313035963,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 134.421452,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 154.8404953,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 132.13127300944902,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.7104759435748917,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.08370481465989202,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 132.175988,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 156.6625116,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 131.17509509317586,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.7375894308888178,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.0826254838449886,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "fa5790dfeefd2c4631ae3c1976506ba1a9dcc7ac",
          "message": "Bump cachix/cachix-action from 14 to 15 (#158)\n\nBumps [cachix/cachix-action](https://github.com/cachix/cachix-action)\nfrom 14 to 15.\n<details>\n<summary>Release notes</summary>\n<p><em>Sourced from <a\nhref=\"https://github.com/cachix/cachix-action/releases\">cachix/cachix-action's\nreleases</a>.</em></p>\n<blockquote>\n<h2>cachix-action-v15</h2>\n<h2>What's Changed</h2>\n<ul>\n<li>Pass <code>cachixArgs</code> to the daemon by <a\nhref=\"https://github.com/sandydoo\"><code>@​sandydoo</code></a> in <a\nhref=\"https://redirect.github.com/cachix/cachix-action/pull/177\">cachix/cachix-action#177</a></li>\n<li>Support path filtering when using the daemon by <a\nhref=\"https://github.com/sandydoo\"><code>@​sandydoo</code></a> in <a\nhref=\"https://redirect.github.com/cachix/cachix-action/pull/182\">cachix/cachix-action#182</a></li>\n<li>Skip prep steps if using <code>pathsToPush</code> by <a\nhref=\"https://github.com/sandydoo\"><code>@​sandydoo</code></a> in <a\nhref=\"https://redirect.github.com/cachix/cachix-action/pull/180\">cachix/cachix-action#180</a></li>\n<li>store-scan: improve error handling when listing the store fails by\n<a href=\"https://github.com/sandydoo\"><code>@​sandydoo</code></a> in <a\nhref=\"https://redirect.github.com/cachix/cachix-action/pull/183\">cachix/cachix-action#183</a></li>\n</ul>\n<p><strong>Full Changelog</strong>: <a\nhref=\"https://github.com/cachix/cachix-action/compare/v14...v15\">https://github.com/cachix/cachix-action/compare/v14...v15</a></p>\n</blockquote>\n</details>\n<details>\n<summary>Commits</summary>\n<ul>\n<li><a\nhref=\"https://github.com/cachix/cachix-action/commit/ad2ddac53f961de1989924296a1f236fcfbaa4fc\"><code>ad2ddac</code></a>\nMerge pull request <a\nhref=\"https://redirect.github.com/cachix/cachix-action/issues/184\">#184</a>\nfrom cachix/dependabot/github_actions/cachix/install-...</li>\n<li><a\nhref=\"https://github.com/cachix/cachix-action/commit/403a1b5dadbdd11bf4858c4815d44727e02634f2\"><code>403a1b5</code></a>\nchore(deps): bump cachix/install-nix-action from 26 to 27</li>\n<li><a\nhref=\"https://github.com/cachix/cachix-action/commit/132bc971f5f5691aadd0d1de82b438c366178557\"><code>132bc97</code></a>\nMerge pull request <a\nhref=\"https://redirect.github.com/cachix/cachix-action/issues/183\">#183</a>\nfrom cachix/fix-179</li>\n<li><a\nhref=\"https://github.com/cachix/cachix-action/commit/662a8831beb5acada52cd71dc06ef391c85066e6\"><code>662a883</code></a>\nstore-scan: use runner or os temp dirs</li>\n<li><a\nhref=\"https://github.com/cachix/cachix-action/commit/08dcf70a394cd1cef74b9d2ddc5ba3179b2ff586\"><code>08dcf70</code></a>\nstore-scan: improve error handling when store listing fails</li>\n<li><a\nhref=\"https://github.com/cachix/cachix-action/commit/991af99d4b9844e58d4aab92f0c7d8ecf9acbe72\"><code>991af99</code></a>\nSkip prep steps if using <code>pathsToPush</code> (<a\nhref=\"https://redirect.github.com/cachix/cachix-action/issues/180\">#180</a>)</li>\n<li><a\nhref=\"https://github.com/cachix/cachix-action/commit/74587ee9209c44c9cc2da93ee9f97487711bf260\"><code>74587ee</code></a>\nMerge pull request <a\nhref=\"https://redirect.github.com/cachix/cachix-action/issues/182\">#182</a>\nfrom cachix/support-daemon-push-filter</li>\n<li><a\nhref=\"https://github.com/cachix/cachix-action/commit/5ee8857e76faa3dbcbee50e47801c572b7524636\"><code>5ee8857</code></a>\nFix syntax</li>\n<li><a\nhref=\"https://github.com/cachix/cachix-action/commit/7431d10acad17978eb5775c6ded207c34714bda0\"><code>7431d10</code></a>\nUse bash in post-build hook</li>\n<li><a\nhref=\"https://github.com/cachix/cachix-action/commit/ad440c0fec35d27eaeca5a4b88ebfdbbcc706ecf\"><code>ad440c0</code></a>\nMake <code>pushFilter</code> filter out just the positive matches</li>\n<li>Additional commits viewable in <a\nhref=\"https://github.com/cachix/cachix-action/compare/v14...v15\">compare\nview</a></li>\n</ul>\n</details>\n<br />\n\n\n[![Dependabot compatibility\nscore](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=cachix/cachix-action&package-manager=github_actions&previous-version=14&new-version=15)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)\n\nDependabot will resolve any conflicts with this PR as long as you don't\nalter it yourself. You can also trigger a rebase manually by commenting\n`@dependabot rebase`.\n\n[//]: # (dependabot-automerge-start)\n[//]: # (dependabot-automerge-end)\n\n---\n\n<details>\n<summary>Dependabot commands and options</summary>\n<br />\n\nYou can trigger Dependabot actions by commenting on this PR:\n- `@dependabot rebase` will rebase this PR\n- `@dependabot recreate` will recreate this PR, overwriting any edits\nthat have been made to it\n- `@dependabot merge` will merge this PR after your CI passes on it\n- `@dependabot squash and merge` will squash and merge this PR after\nyour CI passes on it\n- `@dependabot cancel merge` will cancel a previously requested merge\nand block automerging\n- `@dependabot reopen` will reopen this PR if it is closed\n- `@dependabot close` will close this PR and stop Dependabot recreating\nit. You can achieve the same result by closing it manually\n- `@dependabot show <dependency name> ignore conditions` will show all\nof the ignore conditions of the specified dependency\n- `@dependabot ignore this major version` will close this PR and stop\nDependabot creating any more for this major version (unless you reopen\nthe PR or upgrade to it yourself)\n- `@dependabot ignore this minor version` will close this PR and stop\nDependabot creating any more for this minor version (unless you reopen\nthe PR or upgrade to it yourself)\n- `@dependabot ignore this dependency` will close this PR and stop\nDependabot creating any more for this dependency (unless you reopen the\nPR or upgrade to it yourself)\n\n\n</details>\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2024-12-11T17:30:47Z",
          "tree_id": "ce083da528d573661d51a8f8df794a6e7e7b3335",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/fa5790dfeefd2c4631ae3c1976506ba1a9dcc7ac"
        },
        "date": 1733938705732,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 57.623196,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 76.2643434,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 59.686986819513734,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.6680031851030463,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.09487671468139453,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 381.28053,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 414.35721259999997,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 370.38555168560686,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.040450697196377,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.08167532672999925,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 127.40604,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 152.069673,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 127.29401415883385,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.7475172534822718,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07005167946851582,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 130.128685,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 154.174959,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 129.22649849874452,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.6134516080068408,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.07292848474083181,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f69bca04d6622449cc080588b01c52f9fe37075b",
          "message": "Bump tokio from 1.37.0 to 1.42.0 (#161)\n\nBumps [tokio](https://github.com/tokio-rs/tokio) from 1.37.0 to 1.42.0.\n<details>\n<summary>Release notes</summary>\n<p><em>Sourced from <a\nhref=\"https://github.com/tokio-rs/tokio/releases\">tokio's\nreleases</a>.</em></p>\n<blockquote>\n<h2>Tokio v1.42.0</h2>\n<h1>1.42.0 (Dec 3rd, 2024)</h1>\n<h3>Added</h3>\n<ul>\n<li>io: add <code>AsyncFd::{try_io, try_io_mut}</code> (<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6967\">#6967</a>)</li>\n</ul>\n<h3>Fixed</h3>\n<ul>\n<li>io: avoid <code>ptr-&gt;ref-&gt;ptr</code> roundtrip in\nRegistrationSet (<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6929\">#6929</a>)</li>\n<li>runtime: do not defer <code>yield_now</code> inside\n<code>block_in_place</code> (<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6999\">#6999</a>)</li>\n</ul>\n<h3>Changes</h3>\n<ul>\n<li>io: simplify io readiness logic (<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6966\">#6966</a>)</li>\n</ul>\n<h3>Documented</h3>\n<ul>\n<li>net: fix docs for <code>tokio::net::unix::{pid_t, gid_t,\nuid_t}</code> (<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6791\">#6791</a>)</li>\n<li>time: fix a typo in <code>Instant</code> docs (<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6982\">#6982</a>)</li>\n</ul>\n<p><a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6791\">#6791</a>:\n<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/pull/6791\">tokio-rs/tokio#6791</a>\n<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6929\">#6929</a>:\n<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/pull/6929\">tokio-rs/tokio#6929</a>\n<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6966\">#6966</a>:\n<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/pull/6966\">tokio-rs/tokio#6966</a>\n<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6967\">#6967</a>:\n<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/pull/6967\">tokio-rs/tokio#6967</a>\n<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6982\">#6982</a>:\n<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/pull/6982\">tokio-rs/tokio#6982</a>\n<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6999\">#6999</a>:\n<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/pull/6999\">tokio-rs/tokio#6999</a></p>\n<h2>Tokio v1.41.1</h2>\n<h1>1.41.1 (Nov 7th, 2024)</h1>\n<h3>Fixed</h3>\n<ul>\n<li>metrics: fix bug with wrong number of buckets for the histogram (<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6957\">#6957</a>)</li>\n<li>net: display <code>net</code> requirement for\n<code>net::UdpSocket</code> in docs (<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6938\">#6938</a>)</li>\n<li>net: fix typo in <code>TcpStream</code> internal comment (<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6944\">#6944</a>)</li>\n</ul>\n<p><a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6957\">#6957</a>:\n<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/pull/6957\">tokio-rs/tokio#6957</a>\n<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6938\">#6938</a>:\n<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/pull/6938\">tokio-rs/tokio#6938</a>\n<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6944\">#6944</a>:\n<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/pull/6944\">tokio-rs/tokio#6944</a></p>\n<h2>Tokio v1.41.0</h2>\n<h1>1.41.0 (Oct 22th, 2024)</h1>\n<h3>Added</h3>\n<ul>\n<li>metrics: stabilize <code>global_queue_depth</code> (<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6854\">#6854</a>,\n<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6918\">#6918</a>)</li>\n<li>net: add conversions for unix <code>SocketAddr</code> (<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6868\">#6868</a>)</li>\n<li>sync: add <code>watch::Sender::sender_count</code> (<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6836\">#6836</a>)</li>\n<li>sync: add <code>mpsc::Receiver::blocking_recv_many</code> (<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6867\">#6867</a>)</li>\n</ul>\n<!-- raw HTML omitted -->\n</blockquote>\n<p>... (truncated)</p>\n</details>\n<details>\n<summary>Commits</summary>\n<ul>\n<li><a\nhref=\"https://github.com/tokio-rs/tokio/commit/bb9d57017e100985f86d8ca41ac105ee9140423e\"><code>bb9d570</code></a>\nchore: prepare Tokio v1.42.0 (<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/7005\">#7005</a>)</li>\n<li><a\nhref=\"https://github.com/tokio-rs/tokio/commit/af9c683d52415bf01709197778d49eec1ff78c44\"><code>af9c683</code></a>\ntests: fix typo in build test instructions (<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/7004\">#7004</a>)</li>\n<li><a\nhref=\"https://github.com/tokio-rs/tokio/commit/4bc5a1a05862bad71f85a38bf3c56f721a97b43f\"><code>4bc5a1a</code></a>\nci: allow Unicode-3.0 license for unicode-ident (<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/7006\">#7006</a>)</li>\n<li><a\nhref=\"https://github.com/tokio-rs/tokio/commit/f8948ea021bd4b5626bbd8dee89bee51ba766326\"><code>f8948ea</code></a>\nruntime: do not defer <code>yield_now</code> inside\n<code>block_in_place</code> (<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6999\">#6999</a>)</li>\n<li><a\nhref=\"https://github.com/tokio-rs/tokio/commit/bce9780dd3127cd937923d975e356299226a39aa\"><code>bce9780</code></a>\ntime: use <code>array::from_fn</code> instead of manually creating array\n(<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/7000\">#7000</a>)</li>\n<li><a\nhref=\"https://github.com/tokio-rs/tokio/commit/38151f30cbfbaddce30961384f5b463b0737a401\"><code>38151f3</code></a>\nreadme: unlist 1.32.x as LTS release (<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6997\">#6997</a>)</li>\n<li><a\nhref=\"https://github.com/tokio-rs/tokio/commit/5dda72d338516d709a866cee8577048ba1ea062c\"><code>5dda72d</code></a>\nci: pin valgrind to rustc 1.82 (<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6998\">#6998</a>)</li>\n<li><a\nhref=\"https://github.com/tokio-rs/tokio/commit/c07257f99f92c5d6773fc0827bcfd77e43f0fd46\"><code>c07257f</code></a>\nio: simplify io readiness logic (<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6966\">#6966</a>)</li>\n<li><a\nhref=\"https://github.com/tokio-rs/tokio/commit/d08578fc9a950c8ee0ef242dbce0fcaaed5e853f\"><code>d08578f</code></a>\ntime: fix a typo in <code>Instant</code> docs (<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6982\">#6982</a>)</li>\n<li><a\nhref=\"https://github.com/tokio-rs/tokio/commit/4047d7962a02722859b0f1c7bbcd85bc1bfc865d\"><code>4047d79</code></a>\nmiri: add annotations for tests with miri ignore (<a\nhref=\"https://redirect.github.com/tokio-rs/tokio/issues/6981\">#6981</a>)</li>\n<li>Additional commits viewable in <a\nhref=\"https://github.com/tokio-rs/tokio/compare/tokio-1.37.0...tokio-1.42.0\">compare\nview</a></li>\n</ul>\n</details>\n<br />\n\n\n[![Dependabot compatibility\nscore](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=tokio&package-manager=cargo&previous-version=1.37.0&new-version=1.42.0)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)\n\nDependabot will resolve any conflicts with this PR as long as you don't\nalter it yourself. You can also trigger a rebase manually by commenting\n`@dependabot rebase`.\n\n[//]: # (dependabot-automerge-start)\n[//]: # (dependabot-automerge-end)\n\n---\n\n<details>\n<summary>Dependabot commands and options</summary>\n<br />\n\nYou can trigger Dependabot actions by commenting on this PR:\n- `@dependabot rebase` will rebase this PR\n- `@dependabot recreate` will recreate this PR, overwriting any edits\nthat have been made to it\n- `@dependabot merge` will merge this PR after your CI passes on it\n- `@dependabot squash and merge` will squash and merge this PR after\nyour CI passes on it\n- `@dependabot cancel merge` will cancel a previously requested merge\nand block automerging\n- `@dependabot reopen` will reopen this PR if it is closed\n- `@dependabot close` will close this PR and stop Dependabot recreating\nit. You can achieve the same result by closing it manually\n- `@dependabot show <dependency name> ignore conditions` will show all\nof the ignore conditions of the specified dependency\n- `@dependabot ignore this major version` will close this PR and stop\nDependabot creating any more for this major version (unless you reopen\nthe PR or upgrade to it yourself)\n- `@dependabot ignore this minor version` will close this PR and stop\nDependabot creating any more for this minor version (unless you reopen\nthe PR or upgrade to it yourself)\n- `@dependabot ignore this dependency` will close this PR and stop\nDependabot creating any more for this dependency (unless you reopen the\nPR or upgrade to it yourself)\n\n\n</details>\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2024-12-11T17:36:35Z",
          "tree_id": "e7aec5b17dede543b4f8f23eeb8ea4ebc21cb6a4",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/f69bca04d6622449cc080588b01c52f9fe37075b"
        },
        "date": 1733939131391,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 58.294846,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 70.55272240000001,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 58.50889692499564,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.5752562357973403,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.08877201198844352,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 379.6435505,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 466.5372177000001,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 374.7441031582322,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.0798637182782613,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.08411887726601888,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 130.257913,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 150.2949168,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 128.8687826186588,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.6860477143232515,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07464849835285492,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 130.241029,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 153.12981489999999,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 129.24371340200463,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.7212407500660163,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.08361184133451371,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": false,
          "id": "d8e7893139682111968d6f850a71642f8bbcdcb4",
          "message": "Bump hyper from 0.14.28 to 0.14.31 (#160)\n\nBumps [hyper](https://github.com/hyperium/hyper) from 0.14.28 to\n0.14.31.\n<details>\n<summary>Release notes</summary>\n<p><em>Sourced from <a\nhref=\"https://github.com/hyperium/hyper/releases\">hyper's\nreleases</a>.</em></p>\n<blockquote>\n<h2>v0.14.31</h2>\n<h2>Bug Fixes</h2>\n<ul>\n<li><strong>http1:</strong> improve performance of parsing sequentially\npartial messages (<a\nhref=\"https://github.com/hyperium/hyper/commit/97b595e5892c239a195b199f9e7910f582351c44\">97b595e</a>)</li>\n</ul>\n<h2>v0.14.30</h2>\n<h2>Bug Fixes</h2>\n<ul>\n<li><strong>http1:</strong> reject final chunked if missing 0 (<a\nhref=\"https://github.com/hyperium/hyper/commit/4a51b2afefcc1373c2a5b834fa0ae8d935dbff46\">4a51b2af</a>)</li>\n</ul>\n<h2>v0.14.29</h2>\n<h2>Bug Fixes</h2>\n<ul>\n<li><strong>http1:</strong> start header read timeout immediately (<a\nhref=\"https://redirect.github.com/hyperium/hyper/issues/3305\">#3305</a>)\n(<a\nhref=\"https://github.com/hyperium/hyper/commit/b5c2592fde5e20d29c69428c85aef3d682ee36bc\">b5c2592f</a>)</li>\n</ul>\n<h2>Features</h2>\n<ul>\n<li><strong>http2:</strong> add config for\n<code>max_local_error_reset_streams</code> in server (<a\nhref=\"https://redirect.github.com/hyperium/hyper/issues/3528\">#3528</a>)\n(<a\nhref=\"https://github.com/hyperium/hyper/commit/dedcb674f35eaec765a42b550caabe6f694d86d1\">dedcb674</a>)</li>\n</ul>\n<h2>New Contributors</h2>\n<ul>\n<li><a href=\"https://github.com/jeromegn\"><code>@​jeromegn</code></a>\nmade their first contribution in <a\nhref=\"https://redirect.github.com/hyperium/hyper/pull/3305\">hyperium/hyper#3305</a></li>\n</ul>\n<p><strong>Full Changelog</strong>: <a\nhref=\"https://github.com/hyperium/hyper/compare/v0.14.28...v0.14.29\">https://github.com/hyperium/hyper/compare/v0.14.28...v0.14.29</a></p>\n</blockquote>\n</details>\n<details>\n<summary>Changelog</summary>\n<p><em>Sourced from <a\nhref=\"https://github.com/hyperium/hyper/blob/v0.14.31/CHANGELOG.md\">hyper's\nchangelog</a>.</em></p>\n<blockquote>\n<h3>v0.14.31 (2024-10-15)</h3>\n<h4>Bug Fixes</h4>\n<ul>\n<li><strong>http1:</strong> improve performance of parsing sequentially\npartial messages (<a\nhref=\"https://github.com/hyperium/hyper/commit/97b595e5892c239a195b199f9e7910f582351c44\">97b595e</a>)</li>\n</ul>\n<h3>v0.14.30 (2024-07-09)</h3>\n<h4>Bug Fixes</h4>\n<ul>\n<li><strong>http1:</strong> reject final chunked if missing 0 (<a\nhref=\"https://github.com/hyperium/hyper/commit/4a51b2afefcc1373c2a5b834fa0ae8d935dbff46\">4a51b2af</a>)</li>\n</ul>\n<h3>v0.14.29 (2024-06-03)</h3>\n<h4>Bug Fixes</h4>\n<ul>\n<li><strong>http1:</strong> start header read timeout immediately (<a\nhref=\"https://redirect.github.com/hyperium/hyper/issues/3305\">#3305</a>)\n(<a\nhref=\"https://github.com/hyperium/hyper/commit/b5c2592fde5e20d29c69428c85aef3d682ee36bc\">b5c2592f</a>)</li>\n</ul>\n<h4>Features</h4>\n<ul>\n<li><strong>http2:</strong> add config for\n<code>max_local_error_reset_streams</code> in server (<a\nhref=\"https://redirect.github.com/hyperium/hyper/issues/3528\">#3528</a>)\n(<a\nhref=\"https://github.com/hyperium/hyper/commit/dedcb674f35eaec765a42b550caabe6f694d86d1\">dedcb674</a>)</li>\n</ul>\n</blockquote>\n</details>\n<details>\n<summary>Commits</summary>\n<ul>\n<li><a\nhref=\"https://github.com/hyperium/hyper/commit/7829148b7f3c271863c983a18c37adf5da35e726\"><code>7829148</code></a>\nv0.14.31</li>\n<li><a\nhref=\"https://github.com/hyperium/hyper/commit/97b595e5892c239a195b199f9e7910f582351c44\"><code>97b595e</code></a>\nperf(http1): improve parsing of sequentially partial messages</li>\n<li><a\nhref=\"https://github.com/hyperium/hyper/commit/739d5e634c8f82e7d0578ca979cb6d434bd682c2\"><code>739d5e6</code></a>\nchore(ci): pin some deps for MSRV job</li>\n<li><a\nhref=\"https://github.com/hyperium/hyper/commit/b7e5b4a280c2bd485d470886c8b9bc5bbec50588\"><code>b7e5b4a</code></a>\nstyle(capi): update format of hyper.h</li>\n<li><a\nhref=\"https://github.com/hyperium/hyper/commit/618a18580ae14776c05ab6789a82b5c43281fe8b\"><code>618a185</code></a>\nv0.14.30</li>\n<li><a\nhref=\"https://github.com/hyperium/hyper/commit/4a51b2afefcc1373c2a5b834fa0ae8d935dbff46\"><code>4a51b2a</code></a>\nfix(http1): reject final chunked if missing 0</li>\n<li><a\nhref=\"https://github.com/hyperium/hyper/commit/c38437d377426e01100e07dc1e735f9c382fc6f5\"><code>c38437d</code></a>\nchore(dependencies): bump minimum tokio (<a\nhref=\"https://redirect.github.com/hyperium/hyper/issues/3664\">#3664</a>)</li>\n<li><a\nhref=\"https://github.com/hyperium/hyper/commit/4e61351c1c07f59f8d7b79dcb37c1384acb9f3cb\"><code>4e61351</code></a>\nv0.14.29</li>\n<li><a\nhref=\"https://github.com/hyperium/hyper/commit/b5c2592fde5e20d29c69428c85aef3d682ee36bc\"><code>b5c2592</code></a>\nfix(http1): start header read timeout immediately (<a\nhref=\"https://redirect.github.com/hyperium/hyper/issues/3305\">#3305</a>)</li>\n<li><a\nhref=\"https://github.com/hyperium/hyper/commit/093665e3c318b1360df5a7338facc2b47ac5b54a\"><code>093665e</code></a>\nrefactor(lib): allow warnings in 0.14.x (<a\nhref=\"https://redirect.github.com/hyperium/hyper/issues/3677\">#3677</a>)</li>\n<li>Additional commits viewable in <a\nhref=\"https://github.com/hyperium/hyper/compare/v0.14.28...v0.14.31\">compare\nview</a></li>\n</ul>\n</details>\n<br />\n\n\n[![Dependabot compatibility\nscore](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=hyper&package-manager=cargo&previous-version=0.14.28&new-version=0.14.31)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)\n\nDependabot will resolve any conflicts with this PR as long as you don't\nalter it yourself. You can also trigger a rebase manually by commenting\n`@dependabot rebase`.\n\n[//]: # (dependabot-automerge-start)\n[//]: # (dependabot-automerge-end)\n\n---\n\n<details>\n<summary>Dependabot commands and options</summary>\n<br />\n\nYou can trigger Dependabot actions by commenting on this PR:\n- `@dependabot rebase` will rebase this PR\n- `@dependabot recreate` will recreate this PR, overwriting any edits\nthat have been made to it\n- `@dependabot merge` will merge this PR after your CI passes on it\n- `@dependabot squash and merge` will squash and merge this PR after\nyour CI passes on it\n- `@dependabot cancel merge` will cancel a previously requested merge\nand block automerging\n- `@dependabot reopen` will reopen this PR if it is closed\n- `@dependabot close` will close this PR and stop Dependabot recreating\nit. You can achieve the same result by closing it manually\n- `@dependabot show <dependency name> ignore conditions` will show all\nof the ignore conditions of the specified dependency\n- `@dependabot ignore this major version` will close this PR and stop\nDependabot creating any more for this major version (unless you reopen\nthe PR or upgrade to it yourself)\n- `@dependabot ignore this minor version` will close this PR and stop\nDependabot creating any more for this minor version (unless you reopen\nthe PR or upgrade to it yourself)\n- `@dependabot ignore this dependency` will close this PR and stop\nDependabot creating any more for this dependency (unless you reopen the\nPR or upgrade to it yourself)\n\n\n</details>\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2024-12-11T17:45:30Z",
          "tree_id": "dbffbb2107e4b26420497cd3649ef3276f63f11d",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/d8e7893139682111968d6f850a71642f8bbcdcb4"
        },
        "date": 1733939700037,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 57.259744,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 70.51473525,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 57.45077845353397,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.5774612608620728,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.08438865153302,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 395.142332,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 472.4653142000001,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 386.834407083171,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.021035823164084,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.0771481865529798,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 132.84546,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 152.5520348,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 131.08856824598507,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.6736702243042316,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07746646828208378,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 131.7859985,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 155.85046715,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 130.6382147395192,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.7208830224299732,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.07690596221670815,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": false,
          "id": "18929d2646b804b678dbb24f6530108a982812dd",
          "message": "Bump arduino/setup-protoc from 2 to 3 (#106)\n\nBumps [arduino/setup-protoc](https://github.com/arduino/setup-protoc)\nfrom 2 to 3.\n<details>\n<summary>Release notes</summary>\n<p><em>Sourced from <a\nhref=\"https://github.com/arduino/setup-protoc/releases\">arduino/setup-protoc's\nreleases</a>.</em></p>\n<blockquote>\n<h2>v3.0.0</h2>\n<h2>What's Changed</h2>\n<ul>\n<li>Correct <code>convetion</code> typo in README by <a\nhref=\"https://github.com/nixpanic\"><code>@​nixpanic</code></a> in <a\nhref=\"https://redirect.github.com/arduino/setup-protoc/pull/91\">arduino/setup-protoc#91</a></li>\n<li>Bump <code>@​babel/traverse</code> from 7.22.1 to 7.23.2 by <a\nhref=\"https://github.com/dependabot\"><code>@​dependabot</code></a> in <a\nhref=\"https://redirect.github.com/arduino/setup-protoc/pull/93\">arduino/setup-protoc#93</a></li>\n<li>Upgrade to node 20 by <a\nhref=\"https://github.com/alessio-perugini\"><code>@​alessio-perugini</code></a>\nin <a\nhref=\"https://redirect.github.com/arduino/setup-protoc/pull/95\">arduino/setup-protoc#95</a></li>\n</ul>\n<h2>New Contributors</h2>\n<ul>\n<li><a href=\"https://github.com/nixpanic\"><code>@​nixpanic</code></a>\nmade their first contribution in <a\nhref=\"https://redirect.github.com/arduino/setup-protoc/pull/91\">arduino/setup-protoc#91</a></li>\n</ul>\n<p><strong>Full Changelog</strong>: <a\nhref=\"https://github.com/arduino/setup-protoc/compare/v2.1.0...v3.0.0\">https://github.com/arduino/setup-protoc/compare/v2.1.0...v3.0.0</a></p>\n<h2>v2.1.0</h2>\n<h2>What's Changed</h2>\n<ul>\n<li>Expose <code>path</code> and <code>version</code> in\n<code>outputs</code> by <a\nhref=\"https://github.com/sebastienvermeille\"><code>@​sebastienvermeille</code></a>\nin <a\nhref=\"https://redirect.github.com/arduino/setup-protoc/pull/89\">arduino/setup-protoc#89</a></li>\n<li>Bump semver from 7.5.1 to 7.5.2 by <a\nhref=\"https://github.com/dependabot\"><code>@​dependabot</code></a> in <a\nhref=\"https://redirect.github.com/arduino/setup-protoc/pull/87\">arduino/setup-protoc#87</a></li>\n<li>bump semver to 7.5.3 by <a\nhref=\"https://github.com/alessio-perugini\"><code>@​alessio-perugini</code></a>\nin <a\nhref=\"https://redirect.github.com/arduino/setup-protoc/pull/90\">arduino/setup-protoc#90</a></li>\n</ul>\n<h2>New Contributors</h2>\n<ul>\n<li><a\nhref=\"https://github.com/sebastienvermeille\"><code>@​sebastienvermeille</code></a>\nmade their first contribution in <a\nhref=\"https://redirect.github.com/arduino/setup-protoc/pull/89\">arduino/setup-protoc#89</a></li>\n</ul>\n<p><strong>Full Changelog</strong>: <a\nhref=\"https://github.com/arduino/setup-protoc/compare/v2.0.0...v2.1.0\">https://github.com/arduino/setup-protoc/compare/v2.0.0...v2.1.0</a></p>\n</blockquote>\n</details>\n<details>\n<summary>Commits</summary>\n<ul>\n<li><a\nhref=\"https://github.com/arduino/setup-protoc/commit/c65c819552d16ad3c9b72d9dfd5ba5237b9c906b\"><code>c65c819</code></a>\nUpgrade to node 20 (<a\nhref=\"https://redirect.github.com/arduino/setup-protoc/issues/95\">#95</a>)</li>\n<li><a\nhref=\"https://github.com/arduino/setup-protoc/commit/52a53b4e2d968277c5c749dac537d0b14a6f5272\"><code>52a53b4</code></a>\nMerge pull request <a\nhref=\"https://redirect.github.com/arduino/setup-protoc/issues/93\">#93</a>\nfrom arduino/dependabot/npm_and_yarn/babel/traverse-7....</li>\n<li><a\nhref=\"https://github.com/arduino/setup-protoc/commit/cf7ab7fe8696fefcafb8135834d49955e824a56b\"><code>cf7ab7f</code></a>\nBump <code>@​babel/traverse</code> from 7.22.1 to 7.23.2</li>\n<li><a\nhref=\"https://github.com/arduino/setup-protoc/commit/e2995ba278e6b4bca9bac954e72667db122abed1\"><code>e2995ba</code></a>\nCorrect <code>convetion</code> typo in README (<a\nhref=\"https://redirect.github.com/arduino/setup-protoc/issues/91\">#91</a>)</li>\n<li>See full diff in <a\nhref=\"https://github.com/arduino/setup-protoc/compare/v2...v3\">compare\nview</a></li>\n</ul>\n</details>\n<br />\n\n\n[![Dependabot compatibility\nscore](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=arduino/setup-protoc&package-manager=github_actions&previous-version=2&new-version=3)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)\n\nYou can trigger a rebase of this PR by commenting `@dependabot rebase`.\n\n[//]: # (dependabot-automerge-start)\n[//]: # (dependabot-automerge-end)\n\n---\n\n<details>\n<summary>Dependabot commands and options</summary>\n<br />\n\nYou can trigger Dependabot actions by commenting on this PR:\n- `@dependabot rebase` will rebase this PR\n- `@dependabot recreate` will recreate this PR, overwriting any edits\nthat have been made to it\n- `@dependabot merge` will merge this PR after your CI passes on it\n- `@dependabot squash and merge` will squash and merge this PR after\nyour CI passes on it\n- `@dependabot cancel merge` will cancel a previously requested merge\nand block automerging\n- `@dependabot reopen` will reopen this PR if it is closed\n- `@dependabot close` will close this PR and stop Dependabot recreating\nit. You can achieve the same result by closing it manually\n- `@dependabot show <dependency name> ignore conditions` will show all\nof the ignore conditions of the specified dependency\n- `@dependabot ignore this major version` will close this PR and stop\nDependabot creating any more for this major version (unless you reopen\nthe PR or upgrade to it yourself)\n- `@dependabot ignore this minor version` will close this PR and stop\nDependabot creating any more for this minor version (unless you reopen\nthe PR or upgrade to it yourself)\n- `@dependabot ignore this dependency` will close this PR and stop\nDependabot creating any more for this dependency (unless you reopen the\nPR or upgrade to it yourself)\n\n\n</details>\n\n> **Note**\n> Automatic rebases have been disabled on this pull request as it has\nbeen open for over 30 days.\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2024-12-11T17:47:24Z",
          "tree_id": "93f320539aa14bbc04a7dae56a0dea0be5ad377d",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/18929d2646b804b678dbb24f6530108a982812dd"
        },
        "date": 1733939770741,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 60.447771,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 76.6450179,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 61.813833536448875,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.637827664353182,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.09273546788047718,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 395.75980100000004,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 434.8567023500001,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 385.81479381536053,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.000375285881205,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.08310853452266098,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 135.47401,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 154.4140136,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 133.2327203956709,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.7717232111071439,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.0818871562555356,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 132.95906250000002,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 158.986665,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 132.5838476355498,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.8241517704199737,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.08036241736777558,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "danieljamesharvey@gmail.com",
            "name": "Daniel Harvey",
            "username": "danieljharvey"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": false,
          "id": "b49e56b25170546763b9b9aa64811d86944591be",
          "message": "Add `redundant-clones` lint rule, remove stuff it finds (#164)\n\n<!-- The PR description should answer 2 (maybe 3) important questions:\n-->\n\n### What\n\nI find this lint rule helpful, popped it on.\n\n### How\n\n<!-- How is it trying to accomplish it (what are the implementation\nsteps)? -->",
          "timestamp": "2024-12-11T17:47:58Z",
          "tree_id": "bf06ea9f8506581bd92c6956041343bd30a896c3",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/b49e56b25170546763b9b9aa64811d86944591be"
        },
        "date": 1733939828414,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 58.108761,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 75.0329708,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 59.774324985294925,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.5955795880146084,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.08918523761803446,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 382.801959,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 418.1964158,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 374.821561107291,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 0.9603305391005392,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.07738632867368228,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 132.237664,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 152.61602595,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 130.412936179808,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.6950850424886426,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07860639717727264,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 129.897926,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 156.0777314,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 129.43764632142413,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.6633980831014412,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.07500188580086013,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": false,
          "id": "5e9eda91ecd1c991b73d3dc0cbd54c61427804db",
          "message": "Bump thiserror from 1.0.59 to 2.0.6 (#163)\n\nBumps [thiserror](https://github.com/dtolnay/thiserror) from 1.0.59 to\n2.0.6.\n<details>\n<summary>Release notes</summary>\n<p><em>Sourced from <a\nhref=\"https://github.com/dtolnay/thiserror/releases\">thiserror's\nreleases</a>.</em></p>\n<blockquote>\n<h2>2.0.6</h2>\n<ul>\n<li>Suppress deprecation warning on generated From impls (<a\nhref=\"https://redirect.github.com/dtolnay/thiserror/issues/396\">#396</a>)</li>\n</ul>\n<h2>2.0.5</h2>\n<ul>\n<li>Prevent deprecation warning on generated impl for deprecated type\n(<a\nhref=\"https://redirect.github.com/dtolnay/thiserror/issues/394\">#394</a>)</li>\n</ul>\n<h2>2.0.4</h2>\n<ul>\n<li>Eliminate needless_lifetimes clippy lint in generated\n<code>From</code> impls (<a\nhref=\"https://redirect.github.com/dtolnay/thiserror/issues/391\">#391</a>,\nthanks <a\nhref=\"https://github.com/matt-phylum\"><code>@​matt-phylum</code></a>)</li>\n</ul>\n<h2>2.0.3</h2>\n<ul>\n<li>Support the same Path field being repeated in both Debug and Display\nrepresentation in error message (<a\nhref=\"https://redirect.github.com/dtolnay/thiserror/issues/383\">#383</a>)</li>\n<li>Improve error message when a format trait used in error message is\nnot implemented by some field (<a\nhref=\"https://redirect.github.com/dtolnay/thiserror/issues/384\">#384</a>)</li>\n</ul>\n<h2>2.0.2</h2>\n<ul>\n<li>Fix hang on invalid input inside #[error(...)] attribute (<a\nhref=\"https://redirect.github.com/dtolnay/thiserror/issues/382\">#382</a>)</li>\n</ul>\n<h2>2.0.1</h2>\n<ul>\n<li>Support errors that contain a dynamically sized final field (<a\nhref=\"https://redirect.github.com/dtolnay/thiserror/issues/375\">#375</a>)</li>\n<li>Improve inference of trait bounds for fields that are interpolated\nmultiple times in an error message (<a\nhref=\"https://redirect.github.com/dtolnay/thiserror/issues/377\">#377</a>)</li>\n</ul>\n<h2>2.0.0</h2>\n<h2>Breaking changes</h2>\n<ul>\n<li>\n<p>Referencing keyword-named fields by a raw identifier like\n<code>{r#type}</code> inside a format string is no longer accepted;\nsimply use the unraw name like <code>{type}</code> (<a\nhref=\"https://redirect.github.com/dtolnay/thiserror/issues/347\">#347</a>)</p>\n<p>This aligns thiserror with the standard library's formatting macros,\nwhich gained support for implicit argument capture later than the\nrelease of this feature in thiserror 1.x.</p>\n<pre lang=\"rust\"><code>#[derive(Error, Debug)]\n#[error(&quot;... {type} ...&quot;)]  // Before: {r#type}\npub struct Error {\n    pub r#type: Type,\n}\n</code></pre>\n</li>\n<li>\n<p>Trait bounds are no longer inferred on fields whose value is shadowed\nby an explicit named argument in a format message (<a\nhref=\"https://redirect.github.com/dtolnay/thiserror/issues/345\">#345</a>)</p>\n<pre lang=\"rust\"><code>// Before: impl&lt;T: Octal&gt; Display for\nError&lt;T&gt;\n// After: impl&lt;T&gt; Display for Error&lt;T&gt;\n#[derive(Error, Debug)]\n#[error(&quot;{thing:o}&quot;, thing = &quot;...&quot;)]\npub struct Error&lt;T&gt; {\n    thing: T,\n}\n</code></pre>\n</li>\n<li>\n<p>Tuple structs and tuple variants can no longer use numerical\n<code>{0}</code> <code>{1}</code> access at the same time as supplying\nextra positional arguments for a format message, as this makes it\nambiguous whether the number refers to a tuple field vs a different\npositional arg (<a\nhref=\"https://redirect.github.com/dtolnay/thiserror/issues/354\">#354</a>)</p>\n<pre lang=\"rust\"><code></code></pre>\n</li>\n</ul>\n<!-- raw HTML omitted -->\n</blockquote>\n<p>... (truncated)</p>\n</details>\n<details>\n<summary>Commits</summary>\n<ul>\n<li><a\nhref=\"https://github.com/dtolnay/thiserror/commit/2075e87257e5bfe458cc01bc3764dcd499db254d\"><code>2075e87</code></a>\nRelease 2.0.6</li>\n<li><a\nhref=\"https://github.com/dtolnay/thiserror/commit/e9a90851502686c39a7164f67b296b579445f9ff\"><code>e9a9085</code></a>\nMerge pull request <a\nhref=\"https://redirect.github.com/dtolnay/thiserror/issues/396\">#396</a>\nfrom dtolnay/deprecatedfrom</li>\n<li><a\nhref=\"https://github.com/dtolnay/thiserror/commit/6e8c7244c9b396f0c194a660fcb4bcff75e50b08\"><code>6e8c724</code></a>\nSuppress deprecation warning on generated From impls</li>\n<li><a\nhref=\"https://github.com/dtolnay/thiserror/commit/caf585c978f43ef0348dcd669af301e7a7e8578b\"><code>caf585c</code></a>\nAdd test of deprecated type in From impl</li>\n<li><a\nhref=\"https://github.com/dtolnay/thiserror/commit/f1f159d7e759e986335ed14dd362eb7d6c9815d4\"><code>f1f159d</code></a>\nRelease 2.0.5</li>\n<li><a\nhref=\"https://github.com/dtolnay/thiserror/commit/366a7b253e4b363dfe7204fea7b2e088a81bf8ee\"><code>366a7b2</code></a>\nMerge pull request <a\nhref=\"https://redirect.github.com/dtolnay/thiserror/issues/395\">#395</a>\nfrom dtolnay/fallback</li>\n<li><a\nhref=\"https://github.com/dtolnay/thiserror/commit/88a46035e1bbb17ada53acf28ed7b7b00a0b049a\"><code>88a4603</code></a>\nMove fallback expansion to separate module</li>\n<li><a\nhref=\"https://github.com/dtolnay/thiserror/commit/6712f8cca6cd37c165da4a45ad8820fc97710116\"><code>6712f8c</code></a>\nMerge pull request <a\nhref=\"https://redirect.github.com/dtolnay/thiserror/issues/394\">#394</a>\nfrom dtolnay/deprecated</li>\n<li><a\nhref=\"https://github.com/dtolnay/thiserror/commit/07e7d990facdfdbd559df323dae1f5deb1ba9ab6\"><code>07e7d99</code></a>\nAdd &quot;in this derive macro expansion&quot; to missing Display\nerrors</li>\n<li><a\nhref=\"https://github.com/dtolnay/thiserror/commit/714229d8214e77df019254afbbca18f1c921737e\"><code>714229d</code></a>\nWork around deprecation warning on generated impl for deprecated\ntype</li>\n<li>Additional commits viewable in <a\nhref=\"https://github.com/dtolnay/thiserror/compare/1.0.59...2.0.6\">compare\nview</a></li>\n</ul>\n</details>\n<br />\n\n\n[![Dependabot compatibility\nscore](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=thiserror&package-manager=cargo&previous-version=1.0.59&new-version=2.0.6)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)\n\nDependabot will resolve any conflicts with this PR as long as you don't\nalter it yourself. You can also trigger a rebase manually by commenting\n`@dependabot rebase`.\n\n[//]: # (dependabot-automerge-start)\n[//]: # (dependabot-automerge-end)\n\n---\n\n<details>\n<summary>Dependabot commands and options</summary>\n<br />\n\nYou can trigger Dependabot actions by commenting on this PR:\n- `@dependabot rebase` will rebase this PR\n- `@dependabot recreate` will recreate this PR, overwriting any edits\nthat have been made to it\n- `@dependabot merge` will merge this PR after your CI passes on it\n- `@dependabot squash and merge` will squash and merge this PR after\nyour CI passes on it\n- `@dependabot cancel merge` will cancel a previously requested merge\nand block automerging\n- `@dependabot reopen` will reopen this PR if it is closed\n- `@dependabot close` will close this PR and stop Dependabot recreating\nit. You can achieve the same result by closing it manually\n- `@dependabot show <dependency name> ignore conditions` will show all\nof the ignore conditions of the specified dependency\n- `@dependabot ignore this major version` will close this PR and stop\nDependabot creating any more for this major version (unless you reopen\nthe PR or upgrade to it yourself)\n- `@dependabot ignore this minor version` will close this PR and stop\nDependabot creating any more for this minor version (unless you reopen\nthe PR or upgrade to it yourself)\n- `@dependabot ignore this dependency` will close this PR and stop\nDependabot creating any more for this dependency (unless you reopen the\nPR or upgrade to it yourself)\n\n\n</details>\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2024-12-11T17:49:02Z",
          "tree_id": "c1a034dcd98ce5d0f557f3c7628837401cfdbec9",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/5e9eda91ecd1c991b73d3dc0cbd54c61427804db"
        },
        "date": 1733939915530,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 56.99282,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 72.800465,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 58.190440611241094,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.6166011907504299,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.09441983138356037,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 380.964425,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 413.02641090000003,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 370.53378374319806,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 0.9675927402159914,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.08058543607867424,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 128.55803350000002,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 153.13569479999998,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 128.54989269881892,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.6023018543305341,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.06988864173177625,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 130.425391,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 149.07370350000002,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 128.4253610169376,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.7227928911507604,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.0785805702475801,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": false,
          "id": "025960a725e7b5b9d0c7668951817b60aa70d0f2",
          "message": "Bump sqlformat from 0.2.3 to 0.3.3 (#159)\n\nBumps [sqlformat](https://github.com/shssoichiro/sqlformat-rs) from\n0.2.3 to 0.3.3.\n<details>\n<summary>Release notes</summary>\n<p><em>Sourced from <a\nhref=\"https://github.com/shssoichiro/sqlformat-rs/releases\">sqlformat's\nreleases</a>.</em></p>\n<blockquote>\n<h2>Version 0.3.3</h2>\n<ul>\n<li>Reduce binary size by removing regex dependency (<a\nhref=\"https://redirect.github.com/shssoichiro/sqlformat-rs/issues/68\">#68</a>)</li>\n</ul>\n<h2>Version 0.3.0</h2>\n<ul>\n<li>[breaking] fix: Ignore keywords for uppercase=True (<a\nhref=\"https://redirect.github.com/shssoichiro/sqlformat-rs/issues/53\">#53</a>)</li>\n<li>fix: uppercase=false does not lowercase the query (<a\nhref=\"https://redirect.github.com/shssoichiro/sqlformat-rs/issues/50\">#50</a>)</li>\n<li>fix: Possible to provide an escape hatch for expressions (<a\nhref=\"https://redirect.github.com/shssoichiro/sqlformat-rs/issues/51\">#51</a>)</li>\n</ul>\n<h2>Version 0.2.6</h2>\n<ul>\n<li>fix: ON UPDATE with two many blank formatted incorrectly (<a\nhref=\"https://redirect.github.com/shssoichiro/sqlformat-rs/issues/46\">#46</a>)</li>\n<li>fix: <code>EXCEPT</code> not handled well</li>\n<li>fix: REFERENCES xyz ON UPDATE .. causes formatter to treat the\nremaining as an UPDATE statement</li>\n<li>fix: Escaped strings formatted incorrectly</li>\n<li>fix: RETURNING is not placed on a new line</li>\n<li>fix: fix the issue of misaligned comments after formatting (<a\nhref=\"https://redirect.github.com/shssoichiro/sqlformat-rs/issues/40\">#40</a>)</li>\n</ul>\n</blockquote>\n</details>\n<details>\n<summary>Changelog</summary>\n<p><em>Sourced from <a\nhref=\"https://github.com/shssoichiro/sqlformat-rs/blob/master/CHANGELOG.md\">sqlformat's\nchangelog</a>.</em></p>\n<blockquote>\n<h3>Version 0.3.3</h3>\n<ul>\n<li>Reduce binary size by removing regex dependency (<a\nhref=\"https://redirect.github.com/shssoichiro/sqlformat-rs/issues/68\">#68</a>)</li>\n</ul>\n<h3>Version 0.3.2</h3>\n<ul>\n<li>support ClickHouse/DuckDB join variants</li>\n<li>handle double colons better</li>\n</ul>\n<h3>Version 0.3.1</h3>\n<ul>\n<li>Allow latest regex version (<a\nhref=\"https://redirect.github.com/shssoichiro/sqlformat-rs/issues/55\">#55</a>)\n<ul>\n<li>[slightly breaking] Increases minimum Rust version to 1.65</li>\n</ul>\n</li>\n<li>Fixes for operator parsing (<a\nhref=\"https://redirect.github.com/shssoichiro/sqlformat-rs/issues/57\">#57</a>)</li>\n<li>Performance improvements (<a\nhref=\"https://redirect.github.com/shssoichiro/sqlformat-rs/issues/58\">#58</a>)</li>\n</ul>\n<h3>Version 0.3.0</h3>\n<ul>\n<li>[breaking] fix: Ignore keywords for uppercase=True (<a\nhref=\"https://redirect.github.com/shssoichiro/sqlformat-rs/issues/53\">#53</a>)</li>\n<li>fix: uppercase=false does not lowercase the query (<a\nhref=\"https://redirect.github.com/shssoichiro/sqlformat-rs/issues/50\">#50</a>)</li>\n<li>fix: Possible to provide an escape hatch for expressions (<a\nhref=\"https://redirect.github.com/shssoichiro/sqlformat-rs/issues/51\">#51</a>)</li>\n</ul>\n<h3>Version 0.2.6</h3>\n<ul>\n<li>fix: ON UPDATE with two many blank formatted incorrectly (<a\nhref=\"https://redirect.github.com/shssoichiro/sqlformat-rs/issues/46\">#46</a>)</li>\n<li>fix: <code>EXCEPT</code> not handled well</li>\n<li>fix: REFERENCES xyz ON UPDATE .. causes formatter to treat the\nremaining as an UPDATE statement</li>\n<li>fix: Escaped strings formatted incorrectly</li>\n<li>fix: RETURNING is not placed on a new line</li>\n<li>fix: fix the issue of misaligned comments after formatting (<a\nhref=\"https://redirect.github.com/shssoichiro/sqlformat-rs/issues/40\">#40</a>)</li>\n</ul>\n<h3>Version 0.2.4</h3>\n<ul>\n<li>Remove <code>itertools</code> dependency <a\nhref=\"https://redirect.github.com/shssoichiro/sqlformat-rs/pull/34\">#34</a></li>\n</ul>\n</blockquote>\n</details>\n<details>\n<summary>Commits</summary>\n<ul>\n<li><a\nhref=\"https://github.com/shssoichiro/sqlformat-rs/commit/8f8d29e12537351d793103b82e22316a6bded387\"><code>8f8d29e</code></a>\nVersion 0.3.3</li>\n<li><a\nhref=\"https://github.com/shssoichiro/sqlformat-rs/commit/b5b483fd01f777fb93da3e0b1040d8638081d179\"><code>b5b483f</code></a>\nMerge pull request <a\nhref=\"https://redirect.github.com/shssoichiro/sqlformat-rs/issues/68\">#68</a>\nfrom magic-akari/master</li>\n<li><a\nhref=\"https://github.com/shssoichiro/sqlformat-rs/commit/cfafdb8bea284a5861643c8337fd8dd927fdce25\"><code>cfafdb8</code></a>\nchore: clean code</li>\n<li><a\nhref=\"https://github.com/shssoichiro/sqlformat-rs/commit/8842dc4b3af0b63fde8970a692f13dc6d40bdd6b\"><code>8842dc4</code></a>\nchore: clean code</li>\n<li><a\nhref=\"https://github.com/shssoichiro/sqlformat-rs/commit/1740801d4503305ce80879994d2487e9eb66321a\"><code>1740801</code></a>\nrefactor: Remove regex dependency by using state machine</li>\n<li><a\nhref=\"https://github.com/shssoichiro/sqlformat-rs/commit/244ecb6514ce8bbab951f36ddd9e67e2628622d9\"><code>244ecb6</code></a>\nVersion 0.3.2</li>\n<li><a\nhref=\"https://github.com/shssoichiro/sqlformat-rs/commit/827d639bef94d8e5a5a0e29b41185c8d572f24e6\"><code>827d639</code></a>\nsupport ClickHouse/DuckDB join variants</li>\n<li><a\nhref=\"https://github.com/shssoichiro/sqlformat-rs/commit/fe382a4142d0d1ee470b761a2e506c5e4cdc6dad\"><code>fe382a4</code></a>\nhandle_double_colons (<a\nhref=\"https://redirect.github.com/shssoichiro/sqlformat-rs/issues/63\">#63</a>)</li>\n<li><a\nhref=\"https://github.com/shssoichiro/sqlformat-rs/commit/aa5b1453dbd936c889d0ad7ef61245320a0b72c5\"><code>aa5b145</code></a>\nVersion 0.3.1</li>\n<li><a\nhref=\"https://github.com/shssoichiro/sqlformat-rs/commit/3c008712dfd2b703108fa486713f1e0b67b8be67\"><code>3c00871</code></a>\noptimize matching (<a\nhref=\"https://redirect.github.com/shssoichiro/sqlformat-rs/issues/58\">#58</a>)</li>\n<li>Additional commits viewable in <a\nhref=\"https://github.com/shssoichiro/sqlformat-rs/compare/v0.2.3...v0.3.3\">compare\nview</a></li>\n</ul>\n</details>\n<br />\n\n\n[![Dependabot compatibility\nscore](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=sqlformat&package-manager=cargo&previous-version=0.2.3&new-version=0.3.3)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)\n\nDependabot will resolve any conflicts with this PR as long as you don't\nalter it yourself. You can also trigger a rebase manually by commenting\n`@dependabot rebase`.\n\n[//]: # (dependabot-automerge-start)\n[//]: # (dependabot-automerge-end)\n\n---\n\n<details>\n<summary>Dependabot commands and options</summary>\n<br />\n\nYou can trigger Dependabot actions by commenting on this PR:\n- `@dependabot rebase` will rebase this PR\n- `@dependabot recreate` will recreate this PR, overwriting any edits\nthat have been made to it\n- `@dependabot merge` will merge this PR after your CI passes on it\n- `@dependabot squash and merge` will squash and merge this PR after\nyour CI passes on it\n- `@dependabot cancel merge` will cancel a previously requested merge\nand block automerging\n- `@dependabot reopen` will reopen this PR if it is closed\n- `@dependabot close` will close this PR and stop Dependabot recreating\nit. You can achieve the same result by closing it manually\n- `@dependabot show <dependency name> ignore conditions` will show all\nof the ignore conditions of the specified dependency\n- `@dependabot ignore this major version` will close this PR and stop\nDependabot creating any more for this major version (unless you reopen\nthe PR or upgrade to it yourself)\n- `@dependabot ignore this minor version` will close this PR and stop\nDependabot creating any more for this minor version (unless you reopen\nthe PR or upgrade to it yourself)\n- `@dependabot ignore this dependency` will close this PR and stop\nDependabot creating any more for this dependency (unless you reopen the\nPR or upgrade to it yourself)\n\n\n</details>\n\n---------\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>\nCo-authored-by: Daniel Harvey <danieljamesharvey@gmail.com>",
          "timestamp": "2024-12-11T17:50:36Z",
          "tree_id": "89c693eacb9078b983af6f5ff3705720d6aacb93",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/025960a725e7b5b9d0c7668951817b60aa70d0f2"
        },
        "date": 1733939992108,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 58.139208999999994,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 76.25218629999999,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 60.158913639523256,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.6706922195258116,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.09223460004983366,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 378.80828499999996,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 413.05137375000004,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 370.4371109579301,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 0.9836843004467255,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.07885961615791512,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 130.60475,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 150.9492562,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 128.86397713342072,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.682748528552537,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07850351328954118,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 130.19314450000002,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 150.9643254,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 128.73340208219332,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.6821222018678839,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.07535842280409784,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "brandon@codedmart.com",
            "name": "Brandon Martin",
            "username": "codedmart"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "915b1a60e7f0b9081b8a3340f73026a3bc27bcb3",
          "message": "Use execute_query for introspection (#155)\n\n<!-- The PR description should answer 2 (maybe 3) important questions:\n-->\n\n### What\nThe configuration introspection was using `select_first_row` which was\nnot properly streaming all results and had a limits which caused\nintrospections to fail on larger sets of tables.\n\n<!-- What is this PR trying to accomplish (and why, if it's not\nobvious)? -->\n\n### How\nThis removes `select_first_row` and uses `execute_query` to fetch all\nintrospection data.\n\n<!-- How is it trying to accomplish it (what are the implementation\nsteps)? -->\n\n---------\n\nCo-authored-by: Daniel Harvey <danieljamesharvey@gmail.com>",
          "timestamp": "2024-12-11T20:52:22Z",
          "tree_id": "ea8fa70d20e5e98985bc720711e8ad57d544ca7d",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/915b1a60e7f0b9081b8a3340f73026a3bc27bcb3"
        },
        "date": 1733950775605,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 57.890818,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 71.781719,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 58.746348418641496,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.6305660009089848,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.09368710987208412,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 377.617791,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 414.62141940000004,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 368.94127540438166,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.0930119331597439,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.08093381136276792,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 129.7352445,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 153.8474517,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 129.40096926071425,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.657032405688085,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07281458690461957,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 129.826255,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 155.76947255,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 129.3741568256278,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.6065137060763277,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.07177562285339367,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "brandon@codedmart.com",
            "name": "Brandon Martin",
            "username": "codedmart"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "932d0ebf7ed6a499cbafa5b280fb8167b3195d10",
          "message": "Scalar type representation (#165)\n\n<!-- The PR description should answer 2 (maybe 3) important questions:\n-->\n\n### What\nReturn proper representation for scalar types. Still need to verify\nand/or fix serialization of types.\n\n<!-- What is this PR trying to accomplish (and why, if it's not\nobvious)? -->\n\n### How\n\n<!-- How is it trying to accomplish it (what are the implementation\nsteps)? -->\n\n---------\n\nCo-authored-by: Daniel Harvey <danieljamesharvey@gmail.com>",
          "timestamp": "2024-12-12T17:34:59Z",
          "tree_id": "e047669cfbe92be4bf3f4ab22c9d0f324e5c919b",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/932d0ebf7ed6a499cbafa5b280fb8167b3195d10"
        },
        "date": 1734025348795,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 58.128416,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 72.86839814999999,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 58.852662498485266,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.5693886725242194,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.0885524690336741,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 380.707899,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 420.6914432,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 372.1053565613051,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.1661857934013256,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.08105226884094376,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 131.053895,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 156.37827675,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 130.39322508619722,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.6889917905684797,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07881572422135678,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 130.607579,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 152.1837962,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 129.76805792581848,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.5939201029270862,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.07273046019271652,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "brandon@codedmart.com",
            "name": "Brandon Martin",
            "username": "codedmart"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "fe122a360e29050fce72fc11dc69620ed24e0a03",
          "message": "Release v2.0.0 (#166)\n\nRelease version 2.0.0",
          "timestamp": "2024-12-12T18:30:39Z",
          "tree_id": "124a0bcbb38b416c826b6970295d739a198bb4ed",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/fe122a360e29050fce72fc11dc69620ed24e0a03"
        },
        "date": 1734028679733,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 60.7117735,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 79.12742684999999,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 61.86140774038447,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.608740170275432,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.09460014295771352,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 383.03387,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 420.7051387999999,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 374.0009593932207,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.0224755781534896,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.08068251901897401,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 131.685543,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 156.918553,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 132.01094207276697,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.6151442748948455,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07638961029413988,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 132.3975405,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 157.94590275,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 131.55449965586345,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.7541441463152978,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.08286342267324731,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "brandon@codedmart.com",
            "name": "Brandon Martin",
            "username": "codedmart"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9b81c65910d2f631eabc30eb93dfdac124b2da94",
          "message": "Update changelog.md",
          "timestamp": "2024-12-12T11:11:04-08:00",
          "tree_id": "9184f810e2fdf25ec463b2f789fb94aa87f205ad",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/9b81c65910d2f631eabc30eb93dfdac124b2da94"
        },
        "date": 1734030835107,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 57.177142,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 69.15416069999999,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 57.192252639863234,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.6204492008488103,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.09244443706897672,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 386.886487,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 480.420991,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 379.13313212247186,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.0485976959176355,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.08623172758571347,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 129.72463,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 155.1547458,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 129.46985266384468,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.5962502745068718,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.07067831988309564,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 131.599004,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 149.66218020000002,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 129.7502806801004,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.7009925142520785,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.08046816372804481,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "brandon@codedmart.com",
            "name": "Brandon Martin",
            "username": "codedmart"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "06f0593ba983eab6305aa8fb983c103fd76afc3b",
          "message": "Update docs to reflect ADO.NET connection string (#167)\n\n<!-- The PR description should answer 2 (maybe 3) important questions:\n-->\n\n### What\nIn Hasura v2 we supported ODBC connection string for sqlserver, but in\nDDN we support ADO.NET connection strings.\n\n<!-- What is this PR trying to accomplish (and why, if it's not\nobvious)? -->\n\n### How\n- Update docs to reflect the connection string format supported\n- Try to detect ODBC format and throw a useful error when initializing\nthe connection\n\n<!-- How is it trying to accomplish it (what are the implementation\nsteps)? -->",
          "timestamp": "2024-12-16T14:15:18Z",
          "tree_id": "87e0c8d8d3cfca46e6894693cda1c1f630be97f2",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/06f0593ba983eab6305aa8fb983c103fd76afc3b"
        },
        "date": 1734359364308,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 59.169472,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 76.25640975,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 60.35524618242404,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.6236204194696029,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.0942514816268205,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 381.897329,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 418.3016755,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 372.22573740097533,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 1.0101397768188463,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.08982531470359639,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 133.474195,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 157.2172148,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 132.07643880888122,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.717095388176773,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.08251855702594749,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 132.5233555,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 154.830987,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 131.33692669218382,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.654034077491616,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.07901358125133763,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "karthikeyan@hasura.io",
            "name": "Karthikeyan Chinnakonda",
            "username": "codingkarthik"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": false,
          "id": "95fffd5b8ea3a13aa84b3f7de674ef72a8617b63",
          "message": "Update connector packaging to support Inline Binary definitions (#182)\n\n# Update connector packaging to support multi-platform CLI binaries\n\nThis PR updates our GitHub Actions workflow to support building and\npackaging CLI binaries for multiple platforms while following the latest\nconnector packaging specification. The changes introduce a more robust\napproach to creating platform-specific CLI binaries and generating the\ncorresponding metadata for the Hasura CLI plugin system.\n\n## Key Changes\n\n### Multi-Platform Binary Support\nThe build workflow now creates platform-specific binaries for:\n- Linux (x86_64 and ARM64)\n- macOS (x86_64 and ARM64) \n- Windows (x86_64)\n\n### Enhanced Build and Packaging Process\nEach binary is properly named and packaged following Hasura's\nconventions, with SHA256 checksums generated for verification. The\nbinaries are combined into a single tarball (cli.tar.gz) that gets\nattached to GitHub releases.\n\n### Updated CLI Initialization\nThe CLI initialization command has been enhanced to support the new\nbinary CLI manifest format. This manifest contains platform-specific\ninformation including download URLs, SHA256 checksums, and binary paths.\nWhen initializing a new connector configuration, the CLI can now embed\nthis manifest into the connector metadata, enabling the Hasura CLI to\nautomatically download and use the correct binary for the user's\nplatform.\n\n### Modernized Metadata Generation\nThe connector metadata generation has been updated to align with the\nlatest specification from the packaging RFC. Notable changes include:\n- Added support for version field in metadata\n- Enhanced environment variable definitions to include required flag\n- Updated command definitions to support more flexible command types\n(string, dockerized, shell script)\n- Implemented the new BinaryInline CLI plugin definition format\n- Added support for native toolchain definitions and documentation links\n\n## Impact\nThese changes make our connector more portable across different\nplatforms while maintaining security through checksum verification. The\nupdated metadata format provides better integration with the Hasura\necosystem and improved user experience through platform-specific binary\nselection.\n\n## Testing\n- Verified binary builds for all supported platforms\n- Tested metadata generation with the new CLI manifest\n- Confirmed tarball creation and manifest combination\n- Validated GitHub release artifacts\n\n## Related\n- RFC:\nhttps://github.com/hasura/ndc-hub/blob/main/rfcs/0011-cli-and-connector-packaging.md",
          "timestamp": "2025-01-24T08:47:26Z",
          "tree_id": "c0732d32d17cb8c4aec53636a2430bc263456f18",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/95fffd5b8ea3a13aa84b3f7de674ef72a8617b63"
        },
        "date": 1737709027381,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 54.4665305,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 70.66936449999999,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 55.861043082577126,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.4811001735383371,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.06133688704532405,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 383.61928350000005,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 418.8222379,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 375.13359180731516,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 0.9408964294872817,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.06776240610889064,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 127.999221,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 151.33040599999998,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 127.40564935134452,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.5248125199371714,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.05902503351017756,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 130.9748545,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 153.93904199999997,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 130.28936023111646,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.641019377128174,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.06836659960105453,
            "unit": "ms"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "karthikeyan@hasura.io",
            "name": "Karthikeyan Chinnakonda",
            "username": "codingkarthik"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1be30b6d155acb6820e915b9daabb57474726685",
          "message": "Release v2.0.1 (#183)\n\n<!-- The PR description should answer 2 (maybe 3) important questions:\n-->\n\n### What\n\n<!-- What is this PR trying to accomplish (and why, if it's not\nobvious)? -->\n\n### How\n\n<!-- How is it trying to accomplish it (what are the implementation\nsteps)? -->",
          "timestamp": "2025-01-24T09:19:04Z",
          "tree_id": "be3785240e7d33015195ba1edd560c19679b7904",
          "url": "https://github.com/hasura/ndc-sqlserver/commit/1be30b6d155acb6820e915b9daabb57474726685"
        },
        "date": 1737710799454,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "select-by-pk - median",
            "value": 57.305442,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - p(95)",
            "value": 77.55864369999999,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - connection acquisition time",
            "value": 59.112952508998056,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - request time - (query + acquisition)",
            "value": 0.5416673356030444,
            "unit": "ms"
          },
          {
            "name": "select-by-pk - processing time",
            "value": 0.06674663533562379,
            "unit": "ms"
          },
          {
            "name": "select-variables - median",
            "value": 380.979404,
            "unit": "ms"
          },
          {
            "name": "select-variables - p(95)",
            "value": 414.176941,
            "unit": "ms"
          },
          {
            "name": "select-variables - connection acquisition time",
            "value": 371.09344426855733,
            "unit": "ms"
          },
          {
            "name": "select-variables - request time - (query + acquisition)",
            "value": 0.9073003812004004,
            "unit": "ms"
          },
          {
            "name": "select-variables - processing time",
            "value": 0.06424286609424618,
            "unit": "ms"
          },
          {
            "name": "select-where - median",
            "value": 127.459792,
            "unit": "ms"
          },
          {
            "name": "select-where - p(95)",
            "value": 150.19216899999998,
            "unit": "ms"
          },
          {
            "name": "select-where - connection acquisition time",
            "value": 126.45354733277426,
            "unit": "ms"
          },
          {
            "name": "select-where - request time - (query + acquisition)",
            "value": 0.6085286434832113,
            "unit": "ms"
          },
          {
            "name": "select-where - processing time",
            "value": 0.06066089974196593,
            "unit": "ms"
          },
          {
            "name": "select - median",
            "value": 127.263048,
            "unit": "ms"
          },
          {
            "name": "select - p(95)",
            "value": 148.1320796,
            "unit": "ms"
          },
          {
            "name": "select - connection acquisition time",
            "value": 126.2283144578457,
            "unit": "ms"
          },
          {
            "name": "select - request time - (query + acquisition)",
            "value": 0.5533837399914887,
            "unit": "ms"
          },
          {
            "name": "select - processing time",
            "value": 0.05929295469166173,
            "unit": "ms"
          }
        ]
      }
    ]
  }
}