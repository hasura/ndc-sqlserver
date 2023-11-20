window.BENCHMARK_DATA = {
  "lastUpdate": 1700483937126,
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
      }
    ]
  }
}