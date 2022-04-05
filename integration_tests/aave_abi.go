// Code generated - DO NOT EDIT.
// This file is a generated binding and any manual changes will be lost.

package integration_tests

import (
	"errors"
	"math/big"
	"strings"

	ethereum "github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/event"
)

// Reference imports to suppress errors if they are not otherwise used.
var (
	_ = errors.New
	_ = big.NewInt
	_ = strings.NewReader
	_ = ethereum.NotFound
	_ = bind.Bind
	_ = common.Big1
	_ = types.BloomLookup
	_ = event.NewSubscription
)

// AaveV2MetaData contains all meta data concerning the AaveV2 contract.
var AaveV2MetaData = &bind.MetaData{
	ABI: "[{\"inputs\":[],\"name\":\"accrueFees\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"claimAndUnstake\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"claimed\",\"type\":\"uint256\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"enterStrategy\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[],\"name\":\"mockAccrueFees\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[],\"name\":\"mockClaimAndUnstake\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[],\"name\":\"mockEnterStrategy\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"isPaused\",\"type\":\"bool\"}],\"name\":\"mockPause\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amountOutMinimum\",\"type\":\"uint256\"}],\"name\":\"mockRebalance\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"minAssetsOut\",\"type\":\"uint256\"}],\"name\":\"mockReinvest\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[],\"name\":\"mockRemoveLiquidityRestriction\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[],\"name\":\"mockShutdown\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"}],\"name\":\"mockSweep\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[],\"name\":\"mockTransferFees\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\"},{\"internalType\":\"uint256\",\"name\":\"amountOutMinimum\",\"type\":\"uint256\"}],\"name\":\"rebalance\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\"},{\"internalType\":\"uint256\",\"name\":\"minAssetsOut\",\"type\":\"uint256\"}],\"name\":\"reinvest\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"removeLiquidityRestriction\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"_isPaused\",\"type\":\"bool\"}],\"name\":\"setPause\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"shutdown\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"}],\"name\":\"sweep\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"transferFees\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"isPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"}]",
}

// AaveV2ABI is the input ABI used to generate the binding from.
// Deprecated: Use AaveV2MetaData.ABI instead.
var AaveV2ABI = AaveV2MetaData.ABI

// AaveV2 is an auto generated Go binding around an Ethereum contract.
type AaveV2 struct {
	AaveV2Caller     // Read-only binding to the contract
	AaveV2Transactor // Write-only binding to the contract
	AaveV2Filterer   // Log filterer for contract events
}

// AaveV2Caller is an auto generated read-only Go binding around an Ethereum contract.
type AaveV2Caller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// AaveV2Transactor is an auto generated write-only Go binding around an Ethereum contract.
type AaveV2Transactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// AaveV2Filterer is an auto generated log filtering Go binding around an Ethereum contract events.
type AaveV2Filterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// AaveV2Session is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type AaveV2Session struct {
	Contract     *AaveV2           // Generic contract binding to set the session for
	CallOpts     bind.CallOpts     // Call options to use throughout this session
	TransactOpts bind.TransactOpts // Transaction auth options to use throughout this session
}

// AaveV2CallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type AaveV2CallerSession struct {
	Contract *AaveV2Caller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts // Call options to use throughout this session
}

// AaveV2TransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type AaveV2TransactorSession struct {
	Contract     *AaveV2Transactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts // Transaction auth options to use throughout this session
}

// AaveV2Raw is an auto generated low-level Go binding around an Ethereum contract.
type AaveV2Raw struct {
	Contract *AaveV2 // Generic contract binding to access the raw methods on
}

// AaveV2CallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type AaveV2CallerRaw struct {
	Contract *AaveV2Caller // Generic read-only contract binding to access the raw methods on
}

// AaveV2TransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type AaveV2TransactorRaw struct {
	Contract *AaveV2Transactor // Generic write-only contract binding to access the raw methods on
}

// NewAaveV2 creates a new instance of AaveV2, bound to a specific deployed contract.
func NewAaveV2(address common.Address, backend bind.ContractBackend) (*AaveV2, error) {
	contract, err := bindAaveV2(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &AaveV2{AaveV2Caller: AaveV2Caller{contract: contract}, AaveV2Transactor: AaveV2Transactor{contract: contract}, AaveV2Filterer: AaveV2Filterer{contract: contract}}, nil
}

// NewAaveV2Caller creates a new read-only instance of AaveV2, bound to a specific deployed contract.
func NewAaveV2Caller(address common.Address, caller bind.ContractCaller) (*AaveV2Caller, error) {
	contract, err := bindAaveV2(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &AaveV2Caller{contract: contract}, nil
}

// NewAaveV2Transactor creates a new write-only instance of AaveV2, bound to a specific deployed contract.
func NewAaveV2Transactor(address common.Address, transactor bind.ContractTransactor) (*AaveV2Transactor, error) {
	contract, err := bindAaveV2(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &AaveV2Transactor{contract: contract}, nil
}

// NewAaveV2Filterer creates a new log filterer instance of AaveV2, bound to a specific deployed contract.
func NewAaveV2Filterer(address common.Address, filterer bind.ContractFilterer) (*AaveV2Filterer, error) {
	contract, err := bindAaveV2(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &AaveV2Filterer{contract: contract}, nil
}

// bindAaveV2 binds a generic wrapper to an already deployed contract.
func bindAaveV2(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := abi.JSON(strings.NewReader(AaveV2ABI))
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_AaveV2 *AaveV2Raw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _AaveV2.Contract.AaveV2Caller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_AaveV2 *AaveV2Raw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _AaveV2.Contract.AaveV2Transactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_AaveV2 *AaveV2Raw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _AaveV2.Contract.AaveV2Transactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_AaveV2 *AaveV2CallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _AaveV2.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_AaveV2 *AaveV2TransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _AaveV2.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_AaveV2 *AaveV2TransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _AaveV2.Contract.contract.Transact(opts, method, params...)
}

// IsPaused is a free data retrieval call binding the contract method 0xb187bd26.
//
// Solidity: function isPaused() view returns(bool)
func (_AaveV2 *AaveV2Caller) IsPaused(opts *bind.CallOpts) (bool, error) {
	var out []interface{}
	err := _AaveV2.contract.Call(opts, &out, "isPaused")

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// IsPaused is a free data retrieval call binding the contract method 0xb187bd26.
//
// Solidity: function isPaused() view returns(bool)
func (_AaveV2 *AaveV2Session) IsPaused() (bool, error) {
	return _AaveV2.Contract.IsPaused(&_AaveV2.CallOpts)
}

// IsPaused is a free data retrieval call binding the contract method 0xb187bd26.
//
// Solidity: function isPaused() view returns(bool)
func (_AaveV2 *AaveV2CallerSession) IsPaused() (bool, error) {
	return _AaveV2.Contract.IsPaused(&_AaveV2.CallOpts)
}

// AccrueFees is a paid mutator transaction binding the contract method 0x37a4e834.
//
// Solidity: function accrueFees() returns()
func (_AaveV2 *AaveV2Transactor) AccrueFees(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _AaveV2.contract.Transact(opts, "accrueFees")
}

// AccrueFees is a paid mutator transaction binding the contract method 0x37a4e834.
//
// Solidity: function accrueFees() returns()
func (_AaveV2 *AaveV2Session) AccrueFees() (*types.Transaction, error) {
	return _AaveV2.Contract.AccrueFees(&_AaveV2.TransactOpts)
}

// AccrueFees is a paid mutator transaction binding the contract method 0x37a4e834.
//
// Solidity: function accrueFees() returns()
func (_AaveV2 *AaveV2TransactorSession) AccrueFees() (*types.Transaction, error) {
	return _AaveV2.Contract.AccrueFees(&_AaveV2.TransactOpts)
}

// ClaimAndUnstake is a paid mutator transaction binding the contract method 0xad004e20.
//
// Solidity: function claimAndUnstake() returns(uint256 claimed)
func (_AaveV2 *AaveV2Transactor) ClaimAndUnstake(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _AaveV2.contract.Transact(opts, "claimAndUnstake")
}

// ClaimAndUnstake is a paid mutator transaction binding the contract method 0xad004e20.
//
// Solidity: function claimAndUnstake() returns(uint256 claimed)
func (_AaveV2 *AaveV2Session) ClaimAndUnstake() (*types.Transaction, error) {
	return _AaveV2.Contract.ClaimAndUnstake(&_AaveV2.TransactOpts)
}

// ClaimAndUnstake is a paid mutator transaction binding the contract method 0xad004e20.
//
// Solidity: function claimAndUnstake() returns(uint256 claimed)
func (_AaveV2 *AaveV2TransactorSession) ClaimAndUnstake() (*types.Transaction, error) {
	return _AaveV2.Contract.ClaimAndUnstake(&_AaveV2.TransactOpts)
}

// EnterStrategy is a paid mutator transaction binding the contract method 0x59272e2a.
//
// Solidity: function enterStrategy() returns()
func (_AaveV2 *AaveV2Transactor) EnterStrategy(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _AaveV2.contract.Transact(opts, "enterStrategy")
}

// EnterStrategy is a paid mutator transaction binding the contract method 0x59272e2a.
//
// Solidity: function enterStrategy() returns()
func (_AaveV2 *AaveV2Session) EnterStrategy() (*types.Transaction, error) {
	return _AaveV2.Contract.EnterStrategy(&_AaveV2.TransactOpts)
}

// EnterStrategy is a paid mutator transaction binding the contract method 0x59272e2a.
//
// Solidity: function enterStrategy() returns()
func (_AaveV2 *AaveV2TransactorSession) EnterStrategy() (*types.Transaction, error) {
	return _AaveV2.Contract.EnterStrategy(&_AaveV2.TransactOpts)
}

// Rebalance is a paid mutator transaction binding the contract method 0x2846db02.
//
// Solidity: function rebalance(address[] path, uint256 amountOutMinimum) returns()
func (_AaveV2 *AaveV2Transactor) Rebalance(opts *bind.TransactOpts, path []common.Address, amountOutMinimum *big.Int) (*types.Transaction, error) {
	return _AaveV2.contract.Transact(opts, "rebalance", path, amountOutMinimum)
}

// Rebalance is a paid mutator transaction binding the contract method 0x2846db02.
//
// Solidity: function rebalance(address[] path, uint256 amountOutMinimum) returns()
func (_AaveV2 *AaveV2Session) Rebalance(path []common.Address, amountOutMinimum *big.Int) (*types.Transaction, error) {
	return _AaveV2.Contract.Rebalance(&_AaveV2.TransactOpts, path, amountOutMinimum)
}

// Rebalance is a paid mutator transaction binding the contract method 0x2846db02.
//
// Solidity: function rebalance(address[] path, uint256 amountOutMinimum) returns()
func (_AaveV2 *AaveV2TransactorSession) Rebalance(path []common.Address, amountOutMinimum *big.Int) (*types.Transaction, error) {
	return _AaveV2.Contract.Rebalance(&_AaveV2.TransactOpts, path, amountOutMinimum)
}

// Reinvest is a paid mutator transaction binding the contract method 0x78305dd5.
//
// Solidity: function reinvest(address[] path, uint256 minAssetsOut) returns()
func (_AaveV2 *AaveV2Transactor) Reinvest(opts *bind.TransactOpts, path []common.Address, minAssetsOut *big.Int) (*types.Transaction, error) {
	return _AaveV2.contract.Transact(opts, "reinvest", path, minAssetsOut)
}

// Reinvest is a paid mutator transaction binding the contract method 0x78305dd5.
//
// Solidity: function reinvest(address[] path, uint256 minAssetsOut) returns()
func (_AaveV2 *AaveV2Session) Reinvest(path []common.Address, minAssetsOut *big.Int) (*types.Transaction, error) {
	return _AaveV2.Contract.Reinvest(&_AaveV2.TransactOpts, path, minAssetsOut)
}

// Reinvest is a paid mutator transaction binding the contract method 0x78305dd5.
//
// Solidity: function reinvest(address[] path, uint256 minAssetsOut) returns()
func (_AaveV2 *AaveV2TransactorSession) Reinvest(path []common.Address, minAssetsOut *big.Int) (*types.Transaction, error) {
	return _AaveV2.Contract.Reinvest(&_AaveV2.TransactOpts, path, minAssetsOut)
}

// RemoveLiquidityRestriction is a paid mutator transaction binding the contract method 0x89895950.
//
// Solidity: function removeLiquidityRestriction() returns()
func (_AaveV2 *AaveV2Transactor) RemoveLiquidityRestriction(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _AaveV2.contract.Transact(opts, "removeLiquidityRestriction")
}

// RemoveLiquidityRestriction is a paid mutator transaction binding the contract method 0x89895950.
//
// Solidity: function removeLiquidityRestriction() returns()
func (_AaveV2 *AaveV2Session) RemoveLiquidityRestriction() (*types.Transaction, error) {
	return _AaveV2.Contract.RemoveLiquidityRestriction(&_AaveV2.TransactOpts)
}

// RemoveLiquidityRestriction is a paid mutator transaction binding the contract method 0x89895950.
//
// Solidity: function removeLiquidityRestriction() returns()
func (_AaveV2 *AaveV2TransactorSession) RemoveLiquidityRestriction() (*types.Transaction, error) {
	return _AaveV2.Contract.RemoveLiquidityRestriction(&_AaveV2.TransactOpts)
}

// SetPause is a paid mutator transaction binding the contract method 0xbedb86fb.
//
// Solidity: function setPause(bool _isPaused) returns()
func (_AaveV2 *AaveV2Transactor) SetPause(opts *bind.TransactOpts, _isPaused bool) (*types.Transaction, error) {
	return _AaveV2.contract.Transact(opts, "setPause", _isPaused)
}

// SetPause is a paid mutator transaction binding the contract method 0xbedb86fb.
//
// Solidity: function setPause(bool _isPaused) returns()
func (_AaveV2 *AaveV2Session) SetPause(_isPaused bool) (*types.Transaction, error) {
	return _AaveV2.Contract.SetPause(&_AaveV2.TransactOpts, _isPaused)
}

// SetPause is a paid mutator transaction binding the contract method 0xbedb86fb.
//
// Solidity: function setPause(bool _isPaused) returns()
func (_AaveV2 *AaveV2TransactorSession) SetPause(_isPaused bool) (*types.Transaction, error) {
	return _AaveV2.Contract.SetPause(&_AaveV2.TransactOpts, _isPaused)
}

// Shutdown is a paid mutator transaction binding the contract method 0xfc0e74d1.
//
// Solidity: function shutdown() returns()
func (_AaveV2 *AaveV2Transactor) Shutdown(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _AaveV2.contract.Transact(opts, "shutdown")
}

// Shutdown is a paid mutator transaction binding the contract method 0xfc0e74d1.
//
// Solidity: function shutdown() returns()
func (_AaveV2 *AaveV2Session) Shutdown() (*types.Transaction, error) {
	return _AaveV2.Contract.Shutdown(&_AaveV2.TransactOpts)
}

// Shutdown is a paid mutator transaction binding the contract method 0xfc0e74d1.
//
// Solidity: function shutdown() returns()
func (_AaveV2 *AaveV2TransactorSession) Shutdown() (*types.Transaction, error) {
	return _AaveV2.Contract.Shutdown(&_AaveV2.TransactOpts)
}

// Sweep is a paid mutator transaction binding the contract method 0x01681a62.
//
// Solidity: function sweep(address token) returns()
func (_AaveV2 *AaveV2Transactor) Sweep(opts *bind.TransactOpts, token common.Address) (*types.Transaction, error) {
	return _AaveV2.contract.Transact(opts, "sweep", token)
}

// Sweep is a paid mutator transaction binding the contract method 0x01681a62.
//
// Solidity: function sweep(address token) returns()
func (_AaveV2 *AaveV2Session) Sweep(token common.Address) (*types.Transaction, error) {
	return _AaveV2.Contract.Sweep(&_AaveV2.TransactOpts, token)
}

// Sweep is a paid mutator transaction binding the contract method 0x01681a62.
//
// Solidity: function sweep(address token) returns()
func (_AaveV2 *AaveV2TransactorSession) Sweep(token common.Address) (*types.Transaction, error) {
	return _AaveV2.Contract.Sweep(&_AaveV2.TransactOpts, token)
}

// TransferFees is a paid mutator transaction binding the contract method 0xc2fbe7bc.
//
// Solidity: function transferFees() returns()
func (_AaveV2 *AaveV2Transactor) TransferFees(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _AaveV2.contract.Transact(opts, "transferFees")
}

// TransferFees is a paid mutator transaction binding the contract method 0xc2fbe7bc.
//
// Solidity: function transferFees() returns()
func (_AaveV2 *AaveV2Session) TransferFees() (*types.Transaction, error) {
	return _AaveV2.Contract.TransferFees(&_AaveV2.TransactOpts)
}

// TransferFees is a paid mutator transaction binding the contract method 0xc2fbe7bc.
//
// Solidity: function transferFees() returns()
func (_AaveV2 *AaveV2TransactorSession) TransferFees() (*types.Transaction, error) {
	return _AaveV2.Contract.TransferFees(&_AaveV2.TransactOpts)
}

// AaveV2MockAccrueFeesIterator is returned from FilterMockAccrueFees and is used to iterate over the raw logs and unpacked data for MockAccrueFees events raised by the AaveV2 contract.
type AaveV2MockAccrueFeesIterator struct {
	Event *AaveV2MockAccrueFees // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *AaveV2MockAccrueFeesIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(AaveV2MockAccrueFees)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(AaveV2MockAccrueFees)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *AaveV2MockAccrueFeesIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *AaveV2MockAccrueFeesIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// AaveV2MockAccrueFees represents a MockAccrueFees event raised by the AaveV2 contract.
type AaveV2MockAccrueFees struct {
	Raw types.Log // Blockchain specific contextual infos
}

// FilterMockAccrueFees is a free log retrieval operation binding the contract event 0xeea290d1596cec66f2811f6ec6d4b3e366f8f9b263e15ea87fb624ab69924042.
//
// Solidity: event mockAccrueFees()
func (_AaveV2 *AaveV2Filterer) FilterMockAccrueFees(opts *bind.FilterOpts) (*AaveV2MockAccrueFeesIterator, error) {

	logs, sub, err := _AaveV2.contract.FilterLogs(opts, "mockAccrueFees")
	if err != nil {
		return nil, err
	}
	return &AaveV2MockAccrueFeesIterator{contract: _AaveV2.contract, event: "mockAccrueFees", logs: logs, sub: sub}, nil
}

// WatchMockAccrueFees is a free log subscription operation binding the contract event 0xeea290d1596cec66f2811f6ec6d4b3e366f8f9b263e15ea87fb624ab69924042.
//
// Solidity: event mockAccrueFees()
func (_AaveV2 *AaveV2Filterer) WatchMockAccrueFees(opts *bind.WatchOpts, sink chan<- *AaveV2MockAccrueFees) (event.Subscription, error) {

	logs, sub, err := _AaveV2.contract.WatchLogs(opts, "mockAccrueFees")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(AaveV2MockAccrueFees)
				if err := _AaveV2.contract.UnpackLog(event, "mockAccrueFees", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseMockAccrueFees is a log parse operation binding the contract event 0xeea290d1596cec66f2811f6ec6d4b3e366f8f9b263e15ea87fb624ab69924042.
//
// Solidity: event mockAccrueFees()
func (_AaveV2 *AaveV2Filterer) ParseMockAccrueFees(log types.Log) (*AaveV2MockAccrueFees, error) {
	event := new(AaveV2MockAccrueFees)
	if err := _AaveV2.contract.UnpackLog(event, "mockAccrueFees", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// AaveV2MockClaimAndUnstakeIterator is returned from FilterMockClaimAndUnstake and is used to iterate over the raw logs and unpacked data for MockClaimAndUnstake events raised by the AaveV2 contract.
type AaveV2MockClaimAndUnstakeIterator struct {
	Event *AaveV2MockClaimAndUnstake // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *AaveV2MockClaimAndUnstakeIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(AaveV2MockClaimAndUnstake)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(AaveV2MockClaimAndUnstake)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *AaveV2MockClaimAndUnstakeIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *AaveV2MockClaimAndUnstakeIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// AaveV2MockClaimAndUnstake represents a MockClaimAndUnstake event raised by the AaveV2 contract.
type AaveV2MockClaimAndUnstake struct {
	Raw types.Log // Blockchain specific contextual infos
}

// FilterMockClaimAndUnstake is a free log retrieval operation binding the contract event 0xbfb260c8d19d48ed37a6d7379cdb9622833ea0d1f8389d3b16b0a9dc5cce17b8.
//
// Solidity: event mockClaimAndUnstake()
func (_AaveV2 *AaveV2Filterer) FilterMockClaimAndUnstake(opts *bind.FilterOpts) (*AaveV2MockClaimAndUnstakeIterator, error) {

	logs, sub, err := _AaveV2.contract.FilterLogs(opts, "mockClaimAndUnstake")
	if err != nil {
		return nil, err
	}
	return &AaveV2MockClaimAndUnstakeIterator{contract: _AaveV2.contract, event: "mockClaimAndUnstake", logs: logs, sub: sub}, nil
}

// WatchMockClaimAndUnstake is a free log subscription operation binding the contract event 0xbfb260c8d19d48ed37a6d7379cdb9622833ea0d1f8389d3b16b0a9dc5cce17b8.
//
// Solidity: event mockClaimAndUnstake()
func (_AaveV2 *AaveV2Filterer) WatchMockClaimAndUnstake(opts *bind.WatchOpts, sink chan<- *AaveV2MockClaimAndUnstake) (event.Subscription, error) {

	logs, sub, err := _AaveV2.contract.WatchLogs(opts, "mockClaimAndUnstake")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(AaveV2MockClaimAndUnstake)
				if err := _AaveV2.contract.UnpackLog(event, "mockClaimAndUnstake", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseMockClaimAndUnstake is a log parse operation binding the contract event 0xbfb260c8d19d48ed37a6d7379cdb9622833ea0d1f8389d3b16b0a9dc5cce17b8.
//
// Solidity: event mockClaimAndUnstake()
func (_AaveV2 *AaveV2Filterer) ParseMockClaimAndUnstake(log types.Log) (*AaveV2MockClaimAndUnstake, error) {
	event := new(AaveV2MockClaimAndUnstake)
	if err := _AaveV2.contract.UnpackLog(event, "mockClaimAndUnstake", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// AaveV2MockEnterStrategyIterator is returned from FilterMockEnterStrategy and is used to iterate over the raw logs and unpacked data for MockEnterStrategy events raised by the AaveV2 contract.
type AaveV2MockEnterStrategyIterator struct {
	Event *AaveV2MockEnterStrategy // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *AaveV2MockEnterStrategyIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(AaveV2MockEnterStrategy)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(AaveV2MockEnterStrategy)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *AaveV2MockEnterStrategyIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *AaveV2MockEnterStrategyIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// AaveV2MockEnterStrategy represents a MockEnterStrategy event raised by the AaveV2 contract.
type AaveV2MockEnterStrategy struct {
	Raw types.Log // Blockchain specific contextual infos
}

// FilterMockEnterStrategy is a free log retrieval operation binding the contract event 0x79f347e286799e0fdf15d8e5c56e5ee06f352aefddae4eb82622757f03ea240c.
//
// Solidity: event mockEnterStrategy()
func (_AaveV2 *AaveV2Filterer) FilterMockEnterStrategy(opts *bind.FilterOpts) (*AaveV2MockEnterStrategyIterator, error) {

	logs, sub, err := _AaveV2.contract.FilterLogs(opts, "mockEnterStrategy")
	if err != nil {
		return nil, err
	}
	return &AaveV2MockEnterStrategyIterator{contract: _AaveV2.contract, event: "mockEnterStrategy", logs: logs, sub: sub}, nil
}

// WatchMockEnterStrategy is a free log subscription operation binding the contract event 0x79f347e286799e0fdf15d8e5c56e5ee06f352aefddae4eb82622757f03ea240c.
//
// Solidity: event mockEnterStrategy()
func (_AaveV2 *AaveV2Filterer) WatchMockEnterStrategy(opts *bind.WatchOpts, sink chan<- *AaveV2MockEnterStrategy) (event.Subscription, error) {

	logs, sub, err := _AaveV2.contract.WatchLogs(opts, "mockEnterStrategy")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(AaveV2MockEnterStrategy)
				if err := _AaveV2.contract.UnpackLog(event, "mockEnterStrategy", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseMockEnterStrategy is a log parse operation binding the contract event 0x79f347e286799e0fdf15d8e5c56e5ee06f352aefddae4eb82622757f03ea240c.
//
// Solidity: event mockEnterStrategy()
func (_AaveV2 *AaveV2Filterer) ParseMockEnterStrategy(log types.Log) (*AaveV2MockEnterStrategy, error) {
	event := new(AaveV2MockEnterStrategy)
	if err := _AaveV2.contract.UnpackLog(event, "mockEnterStrategy", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// AaveV2MockPauseIterator is returned from FilterMockPause and is used to iterate over the raw logs and unpacked data for MockPause events raised by the AaveV2 contract.
type AaveV2MockPauseIterator struct {
	Event *AaveV2MockPause // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *AaveV2MockPauseIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(AaveV2MockPause)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(AaveV2MockPause)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *AaveV2MockPauseIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *AaveV2MockPauseIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// AaveV2MockPause represents a MockPause event raised by the AaveV2 contract.
type AaveV2MockPause struct {
	IsPaused bool
	Raw      types.Log // Blockchain specific contextual infos
}

// FilterMockPause is a free log retrieval operation binding the contract event 0xb24266e8968c43c3fcdc4ee6e69d484c1a6bc4fd4d00b54e1d5c75fec4896210.
//
// Solidity: event mockPause(bool isPaused)
func (_AaveV2 *AaveV2Filterer) FilterMockPause(opts *bind.FilterOpts) (*AaveV2MockPauseIterator, error) {

	logs, sub, err := _AaveV2.contract.FilterLogs(opts, "mockPause")
	if err != nil {
		return nil, err
	}
	return &AaveV2MockPauseIterator{contract: _AaveV2.contract, event: "mockPause", logs: logs, sub: sub}, nil
}

// WatchMockPause is a free log subscription operation binding the contract event 0xb24266e8968c43c3fcdc4ee6e69d484c1a6bc4fd4d00b54e1d5c75fec4896210.
//
// Solidity: event mockPause(bool isPaused)
func (_AaveV2 *AaveV2Filterer) WatchMockPause(opts *bind.WatchOpts, sink chan<- *AaveV2MockPause) (event.Subscription, error) {

	logs, sub, err := _AaveV2.contract.WatchLogs(opts, "mockPause")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(AaveV2MockPause)
				if err := _AaveV2.contract.UnpackLog(event, "mockPause", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseMockPause is a log parse operation binding the contract event 0xb24266e8968c43c3fcdc4ee6e69d484c1a6bc4fd4d00b54e1d5c75fec4896210.
//
// Solidity: event mockPause(bool isPaused)
func (_AaveV2 *AaveV2Filterer) ParseMockPause(log types.Log) (*AaveV2MockPause, error) {
	event := new(AaveV2MockPause)
	if err := _AaveV2.contract.UnpackLog(event, "mockPause", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// AaveV2MockRebalanceIterator is returned from FilterMockRebalance and is used to iterate over the raw logs and unpacked data for MockRebalance events raised by the AaveV2 contract.
type AaveV2MockRebalanceIterator struct {
	Event *AaveV2MockRebalance // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *AaveV2MockRebalanceIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(AaveV2MockRebalance)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(AaveV2MockRebalance)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *AaveV2MockRebalanceIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *AaveV2MockRebalanceIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// AaveV2MockRebalance represents a MockRebalance event raised by the AaveV2 contract.
type AaveV2MockRebalance struct {
	Path             []common.Address
	AmountOutMinimum *big.Int
	Raw              types.Log // Blockchain specific contextual infos
}

// FilterMockRebalance is a free log retrieval operation binding the contract event 0x25242bda07a46aa7e9d72c7aca42b90b6883a257ed876327d393c2a1c832eef1.
//
// Solidity: event mockRebalance(address[] path, uint256 amountOutMinimum)
func (_AaveV2 *AaveV2Filterer) FilterMockRebalance(opts *bind.FilterOpts) (*AaveV2MockRebalanceIterator, error) {

	logs, sub, err := _AaveV2.contract.FilterLogs(opts, "mockRebalance")
	if err != nil {
		return nil, err
	}
	return &AaveV2MockRebalanceIterator{contract: _AaveV2.contract, event: "mockRebalance", logs: logs, sub: sub}, nil
}

// WatchMockRebalance is a free log subscription operation binding the contract event 0x25242bda07a46aa7e9d72c7aca42b90b6883a257ed876327d393c2a1c832eef1.
//
// Solidity: event mockRebalance(address[] path, uint256 amountOutMinimum)
func (_AaveV2 *AaveV2Filterer) WatchMockRebalance(opts *bind.WatchOpts, sink chan<- *AaveV2MockRebalance) (event.Subscription, error) {

	logs, sub, err := _AaveV2.contract.WatchLogs(opts, "mockRebalance")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(AaveV2MockRebalance)
				if err := _AaveV2.contract.UnpackLog(event, "mockRebalance", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseMockRebalance is a log parse operation binding the contract event 0x25242bda07a46aa7e9d72c7aca42b90b6883a257ed876327d393c2a1c832eef1.
//
// Solidity: event mockRebalance(address[] path, uint256 amountOutMinimum)
func (_AaveV2 *AaveV2Filterer) ParseMockRebalance(log types.Log) (*AaveV2MockRebalance, error) {
	event := new(AaveV2MockRebalance)
	if err := _AaveV2.contract.UnpackLog(event, "mockRebalance", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// AaveV2MockReinvestIterator is returned from FilterMockReinvest and is used to iterate over the raw logs and unpacked data for MockReinvest events raised by the AaveV2 contract.
type AaveV2MockReinvestIterator struct {
	Event *AaveV2MockReinvest // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *AaveV2MockReinvestIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(AaveV2MockReinvest)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(AaveV2MockReinvest)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *AaveV2MockReinvestIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *AaveV2MockReinvestIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// AaveV2MockReinvest represents a MockReinvest event raised by the AaveV2 contract.
type AaveV2MockReinvest struct {
	Path         []common.Address
	MinAssetsOut *big.Int
	Raw          types.Log // Blockchain specific contextual infos
}

// FilterMockReinvest is a free log retrieval operation binding the contract event 0xa820edc96d666f54cd527363abf610e586001232cbb3a97a3c61f121222032d0.
//
// Solidity: event mockReinvest(address[] path, uint256 minAssetsOut)
func (_AaveV2 *AaveV2Filterer) FilterMockReinvest(opts *bind.FilterOpts) (*AaveV2MockReinvestIterator, error) {

	logs, sub, err := _AaveV2.contract.FilterLogs(opts, "mockReinvest")
	if err != nil {
		return nil, err
	}
	return &AaveV2MockReinvestIterator{contract: _AaveV2.contract, event: "mockReinvest", logs: logs, sub: sub}, nil
}

// WatchMockReinvest is a free log subscription operation binding the contract event 0xa820edc96d666f54cd527363abf610e586001232cbb3a97a3c61f121222032d0.
//
// Solidity: event mockReinvest(address[] path, uint256 minAssetsOut)
func (_AaveV2 *AaveV2Filterer) WatchMockReinvest(opts *bind.WatchOpts, sink chan<- *AaveV2MockReinvest) (event.Subscription, error) {

	logs, sub, err := _AaveV2.contract.WatchLogs(opts, "mockReinvest")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(AaveV2MockReinvest)
				if err := _AaveV2.contract.UnpackLog(event, "mockReinvest", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseMockReinvest is a log parse operation binding the contract event 0xa820edc96d666f54cd527363abf610e586001232cbb3a97a3c61f121222032d0.
//
// Solidity: event mockReinvest(address[] path, uint256 minAssetsOut)
func (_AaveV2 *AaveV2Filterer) ParseMockReinvest(log types.Log) (*AaveV2MockReinvest, error) {
	event := new(AaveV2MockReinvest)
	if err := _AaveV2.contract.UnpackLog(event, "mockReinvest", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// AaveV2MockRemoveLiquidityRestrictionIterator is returned from FilterMockRemoveLiquidityRestriction and is used to iterate over the raw logs and unpacked data for MockRemoveLiquidityRestriction events raised by the AaveV2 contract.
type AaveV2MockRemoveLiquidityRestrictionIterator struct {
	Event *AaveV2MockRemoveLiquidityRestriction // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *AaveV2MockRemoveLiquidityRestrictionIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(AaveV2MockRemoveLiquidityRestriction)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(AaveV2MockRemoveLiquidityRestriction)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *AaveV2MockRemoveLiquidityRestrictionIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *AaveV2MockRemoveLiquidityRestrictionIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// AaveV2MockRemoveLiquidityRestriction represents a MockRemoveLiquidityRestriction event raised by the AaveV2 contract.
type AaveV2MockRemoveLiquidityRestriction struct {
	Raw types.Log // Blockchain specific contextual infos
}

// FilterMockRemoveLiquidityRestriction is a free log retrieval operation binding the contract event 0x357aa2f9a52ebf8e95a37ec4381ec941f12211d04a5874c78159ebcc3d58aa7e.
//
// Solidity: event mockRemoveLiquidityRestriction()
func (_AaveV2 *AaveV2Filterer) FilterMockRemoveLiquidityRestriction(opts *bind.FilterOpts) (*AaveV2MockRemoveLiquidityRestrictionIterator, error) {

	logs, sub, err := _AaveV2.contract.FilterLogs(opts, "mockRemoveLiquidityRestriction")
	if err != nil {
		return nil, err
	}
	return &AaveV2MockRemoveLiquidityRestrictionIterator{contract: _AaveV2.contract, event: "mockRemoveLiquidityRestriction", logs: logs, sub: sub}, nil
}

// WatchMockRemoveLiquidityRestriction is a free log subscription operation binding the contract event 0x357aa2f9a52ebf8e95a37ec4381ec941f12211d04a5874c78159ebcc3d58aa7e.
//
// Solidity: event mockRemoveLiquidityRestriction()
func (_AaveV2 *AaveV2Filterer) WatchMockRemoveLiquidityRestriction(opts *bind.WatchOpts, sink chan<- *AaveV2MockRemoveLiquidityRestriction) (event.Subscription, error) {

	logs, sub, err := _AaveV2.contract.WatchLogs(opts, "mockRemoveLiquidityRestriction")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(AaveV2MockRemoveLiquidityRestriction)
				if err := _AaveV2.contract.UnpackLog(event, "mockRemoveLiquidityRestriction", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseMockRemoveLiquidityRestriction is a log parse operation binding the contract event 0x357aa2f9a52ebf8e95a37ec4381ec941f12211d04a5874c78159ebcc3d58aa7e.
//
// Solidity: event mockRemoveLiquidityRestriction()
func (_AaveV2 *AaveV2Filterer) ParseMockRemoveLiquidityRestriction(log types.Log) (*AaveV2MockRemoveLiquidityRestriction, error) {
	event := new(AaveV2MockRemoveLiquidityRestriction)
	if err := _AaveV2.contract.UnpackLog(event, "mockRemoveLiquidityRestriction", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// AaveV2MockShutdownIterator is returned from FilterMockShutdown and is used to iterate over the raw logs and unpacked data for MockShutdown events raised by the AaveV2 contract.
type AaveV2MockShutdownIterator struct {
	Event *AaveV2MockShutdown // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *AaveV2MockShutdownIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(AaveV2MockShutdown)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(AaveV2MockShutdown)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *AaveV2MockShutdownIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *AaveV2MockShutdownIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// AaveV2MockShutdown represents a MockShutdown event raised by the AaveV2 contract.
type AaveV2MockShutdown struct {
	Raw types.Log // Blockchain specific contextual infos
}

// FilterMockShutdown is a free log retrieval operation binding the contract event 0xf02bc92cc12edf99f533f386e56b3945a3e30921d9f866f5b291b886b0fe0632.
//
// Solidity: event mockShutdown()
func (_AaveV2 *AaveV2Filterer) FilterMockShutdown(opts *bind.FilterOpts) (*AaveV2MockShutdownIterator, error) {

	logs, sub, err := _AaveV2.contract.FilterLogs(opts, "mockShutdown")
	if err != nil {
		return nil, err
	}
	return &AaveV2MockShutdownIterator{contract: _AaveV2.contract, event: "mockShutdown", logs: logs, sub: sub}, nil
}

// WatchMockShutdown is a free log subscription operation binding the contract event 0xf02bc92cc12edf99f533f386e56b3945a3e30921d9f866f5b291b886b0fe0632.
//
// Solidity: event mockShutdown()
func (_AaveV2 *AaveV2Filterer) WatchMockShutdown(opts *bind.WatchOpts, sink chan<- *AaveV2MockShutdown) (event.Subscription, error) {

	logs, sub, err := _AaveV2.contract.WatchLogs(opts, "mockShutdown")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(AaveV2MockShutdown)
				if err := _AaveV2.contract.UnpackLog(event, "mockShutdown", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseMockShutdown is a log parse operation binding the contract event 0xf02bc92cc12edf99f533f386e56b3945a3e30921d9f866f5b291b886b0fe0632.
//
// Solidity: event mockShutdown()
func (_AaveV2 *AaveV2Filterer) ParseMockShutdown(log types.Log) (*AaveV2MockShutdown, error) {
	event := new(AaveV2MockShutdown)
	if err := _AaveV2.contract.UnpackLog(event, "mockShutdown", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// AaveV2MockSweepIterator is returned from FilterMockSweep and is used to iterate over the raw logs and unpacked data for MockSweep events raised by the AaveV2 contract.
type AaveV2MockSweepIterator struct {
	Event *AaveV2MockSweep // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *AaveV2MockSweepIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(AaveV2MockSweep)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(AaveV2MockSweep)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *AaveV2MockSweepIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *AaveV2MockSweepIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// AaveV2MockSweep represents a MockSweep event raised by the AaveV2 contract.
type AaveV2MockSweep struct {
	Token common.Address
	Raw   types.Log // Blockchain specific contextual infos
}

// FilterMockSweep is a free log retrieval operation binding the contract event 0xc352904aa12ccc4e1b1ae66914c29a0b6aa68f6554ab53b0d3943fabe742d78b.
//
// Solidity: event mockSweep(address token)
func (_AaveV2 *AaveV2Filterer) FilterMockSweep(opts *bind.FilterOpts) (*AaveV2MockSweepIterator, error) {

	logs, sub, err := _AaveV2.contract.FilterLogs(opts, "mockSweep")
	if err != nil {
		return nil, err
	}
	return &AaveV2MockSweepIterator{contract: _AaveV2.contract, event: "mockSweep", logs: logs, sub: sub}, nil
}

// WatchMockSweep is a free log subscription operation binding the contract event 0xc352904aa12ccc4e1b1ae66914c29a0b6aa68f6554ab53b0d3943fabe742d78b.
//
// Solidity: event mockSweep(address token)
func (_AaveV2 *AaveV2Filterer) WatchMockSweep(opts *bind.WatchOpts, sink chan<- *AaveV2MockSweep) (event.Subscription, error) {

	logs, sub, err := _AaveV2.contract.WatchLogs(opts, "mockSweep")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(AaveV2MockSweep)
				if err := _AaveV2.contract.UnpackLog(event, "mockSweep", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseMockSweep is a log parse operation binding the contract event 0xc352904aa12ccc4e1b1ae66914c29a0b6aa68f6554ab53b0d3943fabe742d78b.
//
// Solidity: event mockSweep(address token)
func (_AaveV2 *AaveV2Filterer) ParseMockSweep(log types.Log) (*AaveV2MockSweep, error) {
	event := new(AaveV2MockSweep)
	if err := _AaveV2.contract.UnpackLog(event, "mockSweep", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// AaveV2MockTransferFeesIterator is returned from FilterMockTransferFees and is used to iterate over the raw logs and unpacked data for MockTransferFees events raised by the AaveV2 contract.
type AaveV2MockTransferFeesIterator struct {
	Event *AaveV2MockTransferFees // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *AaveV2MockTransferFeesIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(AaveV2MockTransferFees)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(AaveV2MockTransferFees)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *AaveV2MockTransferFeesIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *AaveV2MockTransferFeesIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// AaveV2MockTransferFees represents a MockTransferFees event raised by the AaveV2 contract.
type AaveV2MockTransferFees struct {
	Raw types.Log // Blockchain specific contextual infos
}

// FilterMockTransferFees is a free log retrieval operation binding the contract event 0x9e1dd58aa34e20b52e0befa7246182b86d06fd56ea0a521d4d3dc8d6241cc43a.
//
// Solidity: event mockTransferFees()
func (_AaveV2 *AaveV2Filterer) FilterMockTransferFees(opts *bind.FilterOpts) (*AaveV2MockTransferFeesIterator, error) {

	logs, sub, err := _AaveV2.contract.FilterLogs(opts, "mockTransferFees")
	if err != nil {
		return nil, err
	}
	return &AaveV2MockTransferFeesIterator{contract: _AaveV2.contract, event: "mockTransferFees", logs: logs, sub: sub}, nil
}

// WatchMockTransferFees is a free log subscription operation binding the contract event 0x9e1dd58aa34e20b52e0befa7246182b86d06fd56ea0a521d4d3dc8d6241cc43a.
//
// Solidity: event mockTransferFees()
func (_AaveV2 *AaveV2Filterer) WatchMockTransferFees(opts *bind.WatchOpts, sink chan<- *AaveV2MockTransferFees) (event.Subscription, error) {

	logs, sub, err := _AaveV2.contract.WatchLogs(opts, "mockTransferFees")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(AaveV2MockTransferFees)
				if err := _AaveV2.contract.UnpackLog(event, "mockTransferFees", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseMockTransferFees is a log parse operation binding the contract event 0x9e1dd58aa34e20b52e0befa7246182b86d06fd56ea0a521d4d3dc8d6241cc43a.
//
// Solidity: event mockTransferFees()
func (_AaveV2 *AaveV2Filterer) ParseMockTransferFees(log types.Log) (*AaveV2MockTransferFees, error) {
	event := new(AaveV2MockTransferFees)
	if err := _AaveV2.contract.UnpackLog(event, "mockTransferFees", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}
