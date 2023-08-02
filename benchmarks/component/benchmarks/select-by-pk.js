import {check} from "k6";
import http from "k6/http";

const testid = "select by-pk";
const url = `http://agent:8100/query`;
const data = {
  collection : "Album",
  query : {
    fields : {
      id : {type : "column", column : "AlbumId", arguments : {}},
    },
    where : {
      type : "binary_comparison_operator",
      column : {
        type : "column",
        name : "AlbumId",
        path : [],
      },
      operator : {
        type : "equal",
      },
      value : {
        type : "scalar",
        value : 1,
      },
    },
  },
  arguments : {},
  collection_relationships : {},
};

export default function() {
  const response = http.post(url, JSON.stringify(data), {
    headers : {
      "Content-Type" : "application/json",
    },
  });

  check(response, {
    "status is 200" : (r) => r.status == 200,
  });
}

export const options = {
  tags : {
    testid,
  },
  scenarios : {
    short_sustained : {
      executor : "constant-vus",
      vus : 100,
      duration : "10s",
    },
  },
};
