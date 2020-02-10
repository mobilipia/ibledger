import * as Vega from 'vega-client'
import axios from 'axios'
import * as proto from '../../proto/stubs.js'

const PER_PAGE = 10
const SERVICE_ID = 130
const TX_ID = 0
const TABLE_INDEX = 0
const TimestampEntry = Vega.newType(proto.vega.examples.timestamping.TimestampEntry)

module.exports = {
  install(Vue) {
    Vue.prototype.$blockchain = {
      generateKeyPair() {
        return Vega.keyPair()
      },

      createTimestamp: (keyPair, hash, metadata) => {
        // Describe transaction
        const transaction = Vega.newTransaction({
          author: keyPair.publicKey,
          service_id: SERVICE_ID,
          message_id: TX_ID,
          schema: proto.vega.examples.timestamping.TxTimestamp
        })

        // Transaction data
        const data = {
          content: {
            content_hash: { data: Vega.hexadecimalToUint8Array(hash) },
            metadata: metadata
          }
        }

        // Send transaction into blockchain
        return transaction.send('/api/explorer/v1/transactions', data, keyPair.secretKey)
      },

      getTimestamp: hash => {
        return axios.get(`/api/services/timestamping/v1/timestamps/value?hash=${hash}`).then(response => response.data)
      },

      getTimestampProof: hash => {
        return axios.get('/api/services/configuration/v1/configs/actual').then(response => {
          // actual list of public keys of validators
          const validators = response.data.config.validator_keys.map(validator => validator.consensus_key)

          return axios.get(`/api/services/timestamping/v1/timestamps/proof?hash=${hash}`)
            .then(response => response.data)
            .then(data => {
              return Vega.verifyBlock(data.block_info, validators).then(() => {
                // verify table timestamps in the root tree
                const tableRootHash = Vega.verifyTable(data.state_proof, data.block_info.block.state_hash, SERVICE_ID, TABLE_INDEX)

                // find timestamp in the tree of all timestamps
                const timestampProof = new Vega.MapProof(data.timestamp_proof, Vega.Hash, TimestampEntry)
                if (timestampProof.merkleRoot !== tableRootHash) {
                  throw new Error('Timestamp proof is corrupted')
                }
                const timestampEntry = timestampProof.entries.get(hash)
                if (typeof timestampEntry === 'undefined') {
                  throw new Error('Timestamp not found')
                }

                return {
                  timestamp: {
                    content_hash: Vega.uint8ArrayToHexadecimal(new Uint8Array(timestampEntry.timestamp.content_hash.data)),
                    metadata: timestampEntry.timestamp.metadata
                  },
                  tx_hash: Vega.uint8ArrayToHexadecimal(new Uint8Array(timestampEntry.tx_hash.data)),
                  time: timestampEntry.time
                }
              })
            })
        })
      },

      getBlocks(latest) {
        const suffix = !isNaN(latest) ? '&latest=' + latest : ''
        return axios.get(`/api/explorer/v1/blocks?count=${PER_PAGE}${suffix}`).then(response => response.data)
      },

      getBlock(height) {
        return axios.get(`/api/explorer/v1/block?height=${height}`).then(response => response.data)
      },

      getTransaction(hash) {
        return axios.get(`/api/explorer/v1/transactions?hash=${hash}`).then(response => response.data)
      }
    }
  }
}
