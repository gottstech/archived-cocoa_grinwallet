// Copyright © 2019 vite labs.
//
// ( Origin from https://github.com/vitelabs/Vite_GrinWallet/blob/master/Example/Tests/Tests.swift
//   And refactored & optimized & enhanced by Gotts Developers: https://github.com/gottstech.
// )
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import XCTest
import cocoa_grinwallet

class Tests: XCTestCase {
    
    var firstBridge: GrinBridge!
    var secondBridge: GrinBridge!
    
    override func setUp() {
        super.setUp()
        // Put setup code here. This method is called before the invocation of each test method in the class.
        let libraryDirectory = FileManager.default.urls(for: .libraryDirectory, in: .userDomainMask).first!
        print("wallet data dir: " + libraryDirectory.absoluteString)
        
        let firstURL = libraryDirectory.appendingPathComponent("grin/firstWallet")
        let secondURL = libraryDirectory.appendingPathComponent("grin/secondWallet")
        
        firstBridge = GrinBridge.init(chainType: .floonet, walletUrl: firstURL, password: "1st-grin-wallet")
        secondBridge = GrinBridge.init(chainType: .floonet, walletUrl: secondURL, password: "2nd-grin-wallet")
        
        if !firstBridge.walletExists() {
            // Test wallet recovery from mnemonic phrases
            let result = firstBridge.walletRecovery(phrase: "survey write again earth song palace wreck adjust genre upper arctic episode wonder resource actress eye aspect few element fiscal announce note sword early")
            switch result {
            case .success(_):
                break
            case .failure(let error):
                XCTAssert(false, error.message)
            }
            
            // Restore some balance
            print("try to restore some balance on first wallet")
            var progress = BatchProgress(highestIndex: 0, lastRetrievedIndex: 1)
            repeat {
                let res = firstBridge.walletRestore(start_index: progress.lastRetrievedIndex+1, batch_size: 1000)
                switch res {
                case .success(let restoreProgress):
                    progress = restoreProgress
                    print("wallet restore ok. ", progress)
                    break
                case .failure(let error):
                    XCTAssert(false, error.message)
                    break
                }
            } while progress.lastRetrievedIndex < progress.highestIndex
        } else {
            print("first wallet exist. run a check reparing...")
//            var progress = BatchProgress(highestIndex: 0, lastRetrievedIndex: 1)
//            repeat {
//                let res = firstBridge.walletCheck(start_index: progress.lastRetrievedIndex+1, batch_size: 1000)
//                switch res {
//                case .success(let restoreProgress):
//                    progress = restoreProgress
//                    print("wallet check repair ok. ", progress)
//                    break
//                case .failure(let error):
//                    XCTAssert(false, error.message)
//                    break
//                }
//            } while progress.lastRetrievedIndex < progress.highestIndex
        }
        
        // show the balance of first wallet
        let walletInfo = firstBridge.walletInfo()
        switch walletInfo {
        case .success(let info):
            print("first wallet balance: ", info)
        case .failure(let error):
            XCTAssert(false, error.message)
        }
        
        if !secondBridge.walletExists() {
            // Test wallet initiation from scratch
            print("try to init an empty wallet for second wallet instance")
            let result = secondBridge.walletInit()
            switch result {
            case .success(_):
                break
            case .failure(let error):
                XCTAssert(false, error.message)
            }
        } else {
            print("second wallet exist. run a check reparing...")
//            var progress = BatchProgress(highestIndex: 0, lastRetrievedIndex: 1)
//            repeat {
//                let res = secondBridge.walletCheck(start_index: progress.lastRetrievedIndex+1, batch_size: 1000)
//                switch res {
//                case .success(let restoreProgress):
//                    progress = restoreProgress
//                    print("wallet check repair ok. ", progress)
//                    break
//                case .failure(let error):
//                    XCTAssert(false, error.message)
//                    break
//                }
//            } while progress.lastRetrievedIndex < progress.highestIndex
        }
        // show the balance of second wallet
        let wallet2Info = secondBridge.walletInfo()
        switch wallet2Info {
        case .success(let info):
            print("second wallet balance: ", info)
        case .failure(let error):
            XCTAssert(false, error.message)
        }

    }
    
    override func tearDown() {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
        super.tearDown()
    }
    
    func testGetBalance() {
        let walletInfo = firstBridge.walletInfo()
        switch walletInfo {
        case .success(let info):
            print(info)
        case .failure(let error):
            XCTAssert(false, error.message)
        }
        
        XCTAssert(true, "Pass")
    }
    
    func testGetTxs() {
        let txs = firstBridge.txsGet()
        switch txs {
        case .success(let txs):
            print(txs)
        case .failure(let error):
            XCTAssert(false, error.message)
        }
        
        XCTAssert(true, "Pass")
    }
    
    func testWalletRestore() {
        for bridge in [firstBridge, secondBridge] {
            let result = bridge!.walletRestore(start_index: 1, batch_size: 1000)
            switch result {
            case .success(let progress):
                print("wallet restore ok. ", progress)
                break
            case .failure(let error):
                XCTAssert(false, error.message)
            }
        }
        XCTAssert(true, "Pass")
    }
    
    func testWalletCheck() {
        let result = firstBridge.walletCheck(start_index: 1, batch_size: 1000)
        switch result {
        case .success(let progress):
            print("wallet check repair ok. ", progress)
            XCTAssert(true, "Pass")
        case .failure(let error):
            XCTAssert(false, error.message)
        }
    }
    
    func testTxViaFile() {
        // test sending via file
        let send = firstBridge.txCreate(amount: 10000000, selectionStrategy: "smallest", message: "test tx file send", target_slate_version: -1)
        switch send {
        case .failure(let error):
            XCTAssert(false, error.message)
        case .success(let sendSlate):
            let sendSlateUrl = firstBridge.getSlateUrl(slateId: sendSlate.slateId, isResponse: false)
            do {
                try sendSlate.toJSONString()?.write(to: sendSlateUrl, atomically: true, encoding: .utf8)
                print("send slate in file (for 1st wallet): ", sendSlateUrl)
            } catch {
                XCTAssert(false, error.localizedDescription)
            }
            
            // test receiving by file (on 2nd wallet)
            let receive = secondBridge.txReceive(slateFilePath: sendSlateUrl.path, message: "test tx file receive")
            switch receive {
            case .failure(let error):
                XCTAssert(false, error.message)
            case .success(let receiveSlate):
                let receiveSlateUrl = secondBridge.getSlateUrl(slateId: receiveSlate.slateId, isResponse: true)
                do {
                    try receiveSlate.toJSONString()?.write(to: receiveSlateUrl, atomically: true, encoding: .utf8)
                    print("receive slate in file (for 2nd wallet): ", receiveSlateUrl)
                } catch {
                    XCTAssert(false, error.localizedDescription)
                }
                
                // test tx finalize (receiving by file) on 1st wallet
                let finalize = firstBridge.txFinalize(slateFilePath: receiveSlateUrl.path)
                switch finalize {
                case .success(_):
                    print("finalized slate in file (for 1st wallet): ", receiveSlateUrl)
                    XCTAssert(true, "Pass")
                case .failure(let error):
                    XCTAssert(false, error.message)
                }
            }
        }
    }
    
    func testTxViaHttp() {
        let result = firstBridge.txSend(amount: 10000000, selectionStrategy: "smallest", message: "test tx", dest: "http://127.0.0.1:13415", target_slate_version: -1)
        switch result {
        case .success(_):
            XCTAssert(true, "Pass")
        case .failure(let error):
            XCTAssert(false, error.message)
        }
    }
    
    // no need, already tested in testTxViaFile
//    func testTxReceive() {
//        let receiveSlateUrl = firstBridge.getSlateUrl(slateId: "19b9c0bd-2e7f-47e6-88c7-1c6b76bbe725", isResponse: true)
//        print(receiveSlateUrl.path)
//        let receive = secondBridge.txReceive(slateFilePath: receiveSlateUrl.path, message: "")
//        switch receive {
//        case .success(let receiveSlate):
//            print(receiveSlate)
//        case .failure(let error):
//            XCTAssert(false, error.message)
//        }
//    }
    
    // no need, already tested in testTxViaFile
//    func testTxFinalize() {
//        let receiveSlateUrl = firstBridge.getSlateUrl(slateId: "19b9c0bd-2e7f-47e6-88c7-1c6b76bbe725", isResponse: true)
//        print(receiveSlateUrl.path)
//        let receive = firstBridge.txFinalize(slateFilePath:  receiveSlateUrl.path)
//        switch receive {
//        case .success(let receiveSlate):
//            print(receiveSlate)
//        case .failure(let error):
//            XCTAssert(false, error.message)
//        }
//    }
    
    func testTxCancleAll() {
        let fromNode = firstBridge.txsGet()
        switch fromNode {
        case .success((_, let txs)):
            let sents = txs.filter { $0.txType  == .txSent }
            if sents.isEmpty {
                _ = firstBridge.txCreate(amount: 10000000, selectionStrategy: "smallest", message: "txCreate", target_slate_version: -1)
                testTxCancleAll()
                return
            }
            for sent in sents {
                let cancel = firstBridge.txCancel(id: UInt32(sent.id))
                switch cancel {
                case .success(_):
                    break
                case .failure(let error):
                    XCTAssert(false, error.message)
                }
            }
            XCTAssert(true)
        case .failure(let error):
            XCTAssert(false, error.message)
        }
    }
    
    func testCancelSendTx() {
        let result = firstBridge.txCreate(amount: 20000000, selectionStrategy: "smallest", message: "testCanclesSendTx", target_slate_version: -1)
        switch result {
        case .failure(let error):
            XCTAssert(false, error.message)
        case .success(let slate):
            print("testCancelSendTx - new created tx slateId: ", slate.slateId)
            let txsResult = firstBridge.txsGet()
            switch txsResult {
            case .failure(let error):
                XCTAssert(false, error.message)
            case .success((_, let txs)):
                // find the tx which is just created above
                guard let tx = txs.filter ({
                    return $0.txSlateId == slate.slateId && $0.txType == .txSent
                }).first else {
                    XCTAssert(false); return
                }
                
                // cancel this tx
                let cancleResult = firstBridge.txCancel(id: UInt32(tx.id))
                switch cancleResult {
                case .failure(let error):
                    XCTAssert(false, error.message)
                case .success(_):
                    // check whether it has been cancelled
                    let cancledTxResult = firstBridge.txGet(txSlateId: tx.txSlateId)
                    switch cancledTxResult {
                    case .failure(let error):
                        XCTAssert(false, error.message)
                    case .success((_,let cancledTx)):
                        if cancledTx.txType == .txSentCancelled {
                            XCTAssert(true)
                        } else {
                            XCTAssert(false)
                        }
                    }
                }
            }
        }
    }
    
    func testCancelReceiveTx() {
        let result = firstBridge.txCreate(amount: 20000000, selectionStrategy: "smallest", message: "testCancelReceiveTx-send", target_slate_version: -1)
        switch result {
        case .failure(let error):
            XCTAssert(false, error.message)
        case .success(let sendSlate):
            let sendSlateUrl = firstBridge.getSlateUrl(slateId: sendSlate.slateId, isResponse: false)
            do {
                try sendSlate.toJSONString()?.write(to: sendSlateUrl, atomically: true, encoding: .utf8)
            } catch {
                XCTAssert(false, error.localizedDescription)
            }
            
            // tx receive on 2nd wallet
            let receive = secondBridge.txReceive(slateFilePath: sendSlateUrl.path, message: "testCancelReceiveTx-receive")
            switch receive {
            case .failure(let error):
                XCTAssert(false, error.message)
            case .success(let slate):
                print(slate.slateId)
                let txsResult = secondBridge.txsGet()
                switch txsResult {
                case .failure(let error):
                    XCTAssert(false, error.message)
                case .success((_, let txs)):
                    guard let tx = txs.filter ({
                        return $0.txSlateId == slate.slateId && $0.txType == .txReceived
                    }).first else {
                        XCTAssert(false); return
                    }
                    
                    // cancel this tx on 2nd wallet
                    let cancleResult = secondBridge.txCancel(id: UInt32(tx.id))
                    switch cancleResult {
                    case .failure(let error):
                        XCTAssert(false, error.message)
                    case .success(_):
                        let cancledTxResult = secondBridge.txGet(txSlateId: tx.txSlateId)
                        switch cancledTxResult {
                        case .failure(let error):
                            XCTAssert(false, error.message)
                        case .success((_,let cancledTx)):
                            if cancledTx.txType == .txReceivedCancelled {
                                XCTAssert(true)
                            } else {
                                XCTAssert(false)
                            }
                        }
                    }
                }
            }
        }
    }
    
}
