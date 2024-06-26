import { check } from "k6";
import http from "k6/http";
import { newSummaryHandler } from "../common.js";

const testid = "select-where";
const agentSocket = __ENV.AGENT_SOCKET || "localhost:8080";
const url = `http://${agentSocket}/query`;
const data = {
  collection: "Album",
  query: {
    fields: {
      id: { type: "column", column: "AlbumId", arguments: {} },
      title: { type: "column", column: "Title", arguments: {} },
      artist_id: { type: "column", column: "ArtistId", arguments: {} },
    },
    where: {
      type: "binary_comparison_operator",
      column: {
        type: "column",
        name: "Title",
        path: [],
      },
      operator: {
        type: "other",
        name: "_like",
      },
      value: {
        type: "scalar",
        value: "%a%",
      },
    },
  },
  arguments: {},
  collection_relationships: {},
};

export default function () {
  const response = http.post(url, JSON.stringify(data), {
    headers: {
      "Content-Type": "application/json",
    },
  });

  check(response, {
    "status is 200": (r) => r.status == 200,
  });
}

export const handleSummary = newSummaryHandler(testid);

export const options = {
  tags: {
    testid,
  },
  scenarios: {
    short_sustained: {
      executor: "constant-vus",
      vus: 100,
      duration: "10s",
    },
  },
  thresholds: {
    checks: [
      {
        threshold: "rate == 1",
        abortOnFail: true,
      },
    ],
  },
};
