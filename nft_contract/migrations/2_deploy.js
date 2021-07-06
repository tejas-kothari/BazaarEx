// migrations/2_deploy.js
// SPDX-License-Identifier: MIT
const ERC721PresetMinterPauserAutoId = artifacts.require(
  "ERC721PresetMinterPauserAutoId"
);

module.exports = function (deployer) {
  deployer.deploy(
    ERC721PresetMinterPauserAutoId,
    "My NFT",
    "NFT",
    "https://my-json-server.typicode.com/abcoathup/samplenft/tokens/"
  );
};
