window.BENCHMARK_DATA = {
  "lastUpdate": 1701859370858,
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
      }
    ]
  }
}