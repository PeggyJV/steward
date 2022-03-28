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

// IntegrationTestsMetaData contains all meta data concerning the IntegrationTests contract.
var IntegrationTestsMetaData = &bind.MetaData{
	ABI: "[{\"inputs\":[],\"name\":\"accrueFees\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"claimAndUnstake\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"claimed\",\"type\":\"uint256\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"enterStrategy\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[],\"name\":\"mockAccrueFees\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[],\"name\":\"mockClaimAndUnstake\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[],\"name\":\"mockEnterStrategy\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"isPaused\",\"type\":\"bool\"}],\"name\":\"mockPause\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amountOutMinimum\",\"type\":\"uint256\"}],\"name\":\"mockRebalance\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"minAssetsOut\",\"type\":\"uint256\"}],\"name\":\"mockReinvest\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[],\"name\":\"mockRemoveLiquidityRestriction\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[],\"name\":\"mockShutdown\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"}],\"name\":\"mockSweep\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[],\"name\":\"mockTransferFees\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\"},{\"internalType\":\"uint256\",\"name\":\"amountOutMinimum\",\"type\":\"uint256\"}],\"name\":\"rebalance\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\"},{\"internalType\":\"uint256\",\"name\":\"minAssetsOut\",\"type\":\"uint256\"}],\"name\":\"reinvest\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"removeLiquidityRestriction\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"_isPaused\",\"type\":\"bool\"}],\"name\":\"setPause\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"shutdown\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"}],\"name\":\"sweep\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"transferFees\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"isPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"}]",
}

// IntegrationTestsABI is the input ABI used to generate the binding from.
// Deprecated: Use IntegrationTestsMetaData.ABI instead.
var IntegrationTestsABI = IntegrationTestsMetaData.ABI

// IntegrationTests is an auto generated Go binding around an Ethereum contract.
type IntegrationTests struct {
	IntegrationTestsCaller     // Read-only binding to the contract
	IntegrationTestsTransactor // Write-only binding to the contract
	IntegrationTestsFilterer   // Log filterer for contract events
}

// IntegrationTestsCaller is an auto generated read-only Go binding around an Ethereum contract.
type IntegrationTestsCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// IntegrationTestsTransactor is an auto generated write-only Go binding around an Ethereum contract.
type IntegrationTestsTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// IntegrationTestsFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type IntegrationTestsFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// IntegrationTestsSession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type IntegrationTestsSession struct {
	Contract     *IntegrationTests // Generic contract binding to set the session for
	CallOpts     bind.CallOpts     // Call options to use throughout this session
	TransactOpts bind.TransactOpts // Transaction auth options to use throughout this session
}

// IntegrationTestsCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type IntegrationTestsCallerSession struct {
	Contract *IntegrationTestsCaller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts           // Call options to use throughout this session
}

// IntegrationTestsTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type IntegrationTestsTransactorSession struct {
	Contract     *IntegrationTestsTransactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts           // Transaction auth options to use throughout this session
}

// IntegrationTestsRaw is an auto generated low-level Go binding around an Ethereum contract.
type IntegrationTestsRaw struct {
	Contract *IntegrationTests // Generic contract binding to access the raw methods on
}

// IntegrationTestsCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type IntegrationTestsCallerRaw struct {
	Contract *IntegrationTestsCaller // Generic read-only contract binding to access the raw methods on
}

// IntegrationTestsTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type IntegrationTestsTransactorRaw struct {
	Contract *IntegrationTestsTransactor // Generic write-only contract binding to access the raw methods on
}

// NewIntegrationTests creates a new instance of IntegrationTests, bound to a specific deployed contract.
func NewIntegrationTests(address common.Address, backend bind.ContractBackend) (*IntegrationTests, error) {
	contract, err := bindIntegrationTests(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &IntegrationTests{IntegrationTestsCaller: IntegrationTestsCaller{contract: contract}, IntegrationTestsTransactor: IntegrationTestsTransactor{contract: contract}, IntegrationTestsFilterer: IntegrationTestsFilterer{contract: contract}}, nil
}

// NewIntegrationTestsCaller creates a new read-only instance of IntegrationTests, bound to a specific deployed contract.
func NewIntegrationTestsCaller(address common.Address, caller bind.ContractCaller) (*IntegrationTestsCaller, error) {
	contract, err := bindIntegrationTests(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &IntegrationTestsCaller{contract: contract}, nil
}

// NewIntegrationTestsTransactor creates a new write-only instance of IntegrationTests, bound to a specific deployed contract.
func NewIntegrationTestsTransactor(address common.Address, transactor bind.ContractTransactor) (*IntegrationTestsTransactor, error) {
	contract, err := bindIntegrationTests(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &IntegrationTestsTransactor{contract: contract}, nil
}

// NewIntegrationTestsFilterer creates a new log filterer instance of IntegrationTests, bound to a specific deployed contract.
func NewIntegrationTestsFilterer(address common.Address, filterer bind.ContractFilterer) (*IntegrationTestsFilterer, error) {
	contract, err := bindIntegrationTests(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &IntegrationTestsFilterer{contract: contract}, nil
}

// bindIntegrationTests binds a generic wrapper to an already deployed contract.
func bindIntegrationTests(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := abi.JSON(strings.NewReader(IntegrationTestsABI))
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_IntegrationTests *IntegrationTestsRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _IntegrationTests.Contract.IntegrationTestsCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_IntegrationTests *IntegrationTestsRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _IntegrationTests.Contract.IntegrationTestsTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_IntegrationTests *IntegrationTestsRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _IntegrationTests.Contract.IntegrationTestsTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_IntegrationTests *IntegrationTestsCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _IntegrationTests.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_IntegrationTests *IntegrationTestsTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _IntegrationTests.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_IntegrationTests *IntegrationTestsTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _IntegrationTests.Contract.contract.Transact(opts, method, params...)
}

// IsPaused is a free data retrieval call binding the contract method 0xb187bd26.
//
// Solidity: function isPaused() view returns(bool)
func (_IntegrationTests *IntegrationTestsCaller) IsPaused(opts *bind.CallOpts) (bool, error) {
	var out []interface{}
	err := _IntegrationTests.contract.Call(opts, &out, "isPaused")

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// IsPaused is a free data retrieval call binding the contract method 0xb187bd26.
//
// Solidity: function isPaused() view returns(bool)
func (_IntegrationTests *IntegrationTestsSession) IsPaused() (bool, error) {
	return _IntegrationTests.Contract.IsPaused(&_IntegrationTests.CallOpts)
}

// IsPaused is a free data retrieval call binding the contract method 0xb187bd26.
//
// Solidity: function isPaused() view returns(bool)
func (_IntegrationTests *IntegrationTestsCallerSession) IsPaused() (bool, error) {
	return _IntegrationTests.Contract.IsPaused(&_IntegrationTests.CallOpts)
}

// AccrueFees is a paid mutator transaction binding the contract method 0x37a4e834.
//
// Solidity: function accrueFees() returns()
func (_IntegrationTests *IntegrationTestsTransactor) AccrueFees(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _IntegrationTests.contract.Transact(opts, "accrueFees")
}

// AccrueFees is a paid mutator transaction binding the contract method 0x37a4e834.
//
// Solidity: function accrueFees() returns()
func (_IntegrationTests *IntegrationTestsSession) AccrueFees() (*types.Transaction, error) {
	return _IntegrationTests.Contract.AccrueFees(&_IntegrationTests.TransactOpts)
}

// AccrueFees is a paid mutator transaction binding the contract method 0x37a4e834.
//
// Solidity: function accrueFees() returns()
func (_IntegrationTests *IntegrationTestsTransactorSession) AccrueFees() (*types.Transaction, error) {
	return _IntegrationTests.Contract.AccrueFees(&_IntegrationTests.TransactOpts)
}

// ClaimAndUnstake is a paid mutator transaction binding the contract method 0xad004e20.
//
// Solidity: function claimAndUnstake() returns(uint256 claimed)
func (_IntegrationTests *IntegrationTestsTransactor) ClaimAndUnstake(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _IntegrationTests.contract.Transact(opts, "claimAndUnstake")
}

// ClaimAndUnstake is a paid mutator transaction binding the contract method 0xad004e20.
//
// Solidity: function claimAndUnstake() returns(uint256 claimed)
func (_IntegrationTests *IntegrationTestsSession) ClaimAndUnstake() (*types.Transaction, error) {
	return _IntegrationTests.Contract.ClaimAndUnstake(&_IntegrationTests.TransactOpts)
}

// ClaimAndUnstake is a paid mutator transaction binding the contract method 0xad004e20.
//
// Solidity: function claimAndUnstake() returns(uint256 claimed)
func (_IntegrationTests *IntegrationTestsTransactorSession) ClaimAndUnstake() (*types.Transaction, error) {
	return _IntegrationTests.Contract.ClaimAndUnstake(&_IntegrationTests.TransactOpts)
}

// EnterStrategy is a paid mutator transaction binding the contract method 0x59272e2a.
//
// Solidity: function enterStrategy() returns()
func (_IntegrationTests *IntegrationTestsTransactor) EnterStrategy(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _IntegrationTests.contract.Transact(opts, "enterStrategy")
}

// EnterStrategy is a paid mutator transaction binding the contract method 0x59272e2a.
//
// Solidity: function enterStrategy() returns()
func (_IntegrationTests *IntegrationTestsSession) EnterStrategy() (*types.Transaction, error) {
	return _IntegrationTests.Contract.EnterStrategy(&_IntegrationTests.TransactOpts)
}

// EnterStrategy is a paid mutator transaction binding the contract method 0x59272e2a.
//
// Solidity: function enterStrategy() returns()
func (_IntegrationTests *IntegrationTestsTransactorSession) EnterStrategy() (*types.Transaction, error) {
	return _IntegrationTests.Contract.EnterStrategy(&_IntegrationTests.TransactOpts)
}

// Rebalance is a paid mutator transaction binding the contract method 0x2846db02.
//
// Solidity: function rebalance(address[] path, uint256 amountOutMinimum) returns()
func (_IntegrationTests *IntegrationTestsTransactor) Rebalance(opts *bind.TransactOpts, path []common.Address, amountOutMinimum *big.Int) (*types.Transaction, error) {
	return _IntegrationTests.contract.Transact(opts, "rebalance", path, amountOutMinimum)
}

// Rebalance is a paid mutator transaction binding the contract method 0x2846db02.
//
// Solidity: function rebalance(address[] path, uint256 amountOutMinimum) returns()
func (_IntegrationTests *IntegrationTestsSession) Rebalance(path []common.Address, amountOutMinimum *big.Int) (*types.Transaction, error) {
	return _IntegrationTests.Contract.Rebalance(&_IntegrationTests.TransactOpts, path, amountOutMinimum)
}

// Rebalance is a paid mutator transaction binding the contract method 0x2846db02.
//
// Solidity: function rebalance(address[] path, uint256 amountOutMinimum) returns()
func (_IntegrationTests *IntegrationTestsTransactorSession) Rebalance(path []common.Address, amountOutMinimum *big.Int) (*types.Transaction, error) {
	return _IntegrationTests.Contract.Rebalance(&_IntegrationTests.TransactOpts, path, amountOutMinimum)
}

// Reinvest is a paid mutator transaction binding the contract method 0x78305dd5.
//
// Solidity: function reinvest(address[] path, uint256 minAssetsOut) returns()
func (_IntegrationTests *IntegrationTestsTransactor) Reinvest(opts *bind.TransactOpts, path []common.Address, minAssetsOut *big.Int) (*types.Transaction, error) {
	return _IntegrationTests.contract.Transact(opts, "reinvest", path, minAssetsOut)
}

// Reinvest is a paid mutator transaction binding the contract method 0x78305dd5.
//
// Solidity: function reinvest(address[] path, uint256 minAssetsOut) returns()
func (_IntegrationTests *IntegrationTestsSession) Reinvest(path []common.Address, minAssetsOut *big.Int) (*types.Transaction, error) {
	return _IntegrationTests.Contract.Reinvest(&_IntegrationTests.TransactOpts, path, minAssetsOut)
}

// Reinvest is a paid mutator transaction binding the contract method 0x78305dd5.
//
// Solidity: function reinvest(address[] path, uint256 minAssetsOut) returns()
func (_IntegrationTests *IntegrationTestsTransactorSession) Reinvest(path []common.Address, minAssetsOut *big.Int) (*types.Transaction, error) {
	return _IntegrationTests.Contract.Reinvest(&_IntegrationTests.TransactOpts, path, minAssetsOut)
}

// RemoveLiquidityRestriction is a paid mutator transaction binding the contract method 0x89895950.
//
// Solidity: function removeLiquidityRestriction() returns()
func (_IntegrationTests *IntegrationTestsTransactor) RemoveLiquidityRestriction(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _IntegrationTests.contract.Transact(opts, "removeLiquidityRestriction")
}

// RemoveLiquidityRestriction is a paid mutator transaction binding the contract method 0x89895950.
//
// Solidity: function removeLiquidityRestriction() returns()
func (_IntegrationTests *IntegrationTestsSession) RemoveLiquidityRestriction() (*types.Transaction, error) {
	return _IntegrationTests.Contract.RemoveLiquidityRestriction(&_IntegrationTests.TransactOpts)
}

// RemoveLiquidityRestriction is a paid mutator transaction binding the contract method 0x89895950.
//
// Solidity: function removeLiquidityRestriction() returns()
func (_IntegrationTests *IntegrationTestsTransactorSession) RemoveLiquidityRestriction() (*types.Transaction, error) {
	return _IntegrationTests.Contract.RemoveLiquidityRestriction(&_IntegrationTests.TransactOpts)
}

// SetPause is a paid mutator transaction binding the contract method 0xbedb86fb.
//
// Solidity: function setPause(bool _isPaused) returns()
func (_IntegrationTests *IntegrationTestsTransactor) SetPause(opts *bind.TransactOpts, _isPaused bool) (*types.Transaction, error) {
	return _IntegrationTests.contract.Transact(opts, "setPause", _isPaused)
}

// SetPause is a paid mutator transaction binding the contract method 0xbedb86fb.
//
// Solidity: function setPause(bool _isPaused) returns()
func (_IntegrationTests *IntegrationTestsSession) SetPause(_isPaused bool) (*types.Transaction, error) {
	return _IntegrationTests.Contract.SetPause(&_IntegrationTests.TransactOpts, _isPaused)
}

// SetPause is a paid mutator transaction binding the contract method 0xbedb86fb.
//
// Solidity: function setPause(bool _isPaused) returns()
func (_IntegrationTests *IntegrationTestsTransactorSession) SetPause(_isPaused bool) (*types.Transaction, error) {
	return _IntegrationTests.Contract.SetPause(&_IntegrationTests.TransactOpts, _isPaused)
}

// Shutdown is a paid mutator transaction binding the contract method 0xfc0e74d1.
//
// Solidity: function shutdown() returns()
func (_IntegrationTests *IntegrationTestsTransactor) Shutdown(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _IntegrationTests.contract.Transact(opts, "shutdown")
}

// Shutdown is a paid mutator transaction binding the contract method 0xfc0e74d1.
//
// Solidity: function shutdown() returns()
func (_IntegrationTests *IntegrationTestsSession) Shutdown() (*types.Transaction, error) {
	return _IntegrationTests.Contract.Shutdown(&_IntegrationTests.TransactOpts)
}

// Shutdown is a paid mutator transaction binding the contract method 0xfc0e74d1.
//
// Solidity: function shutdown() returns()
func (_IntegrationTests *IntegrationTestsTransactorSession) Shutdown() (*types.Transaction, error) {
	return _IntegrationTests.Contract.Shutdown(&_IntegrationTests.TransactOpts)
}

// Sweep is a paid mutator transaction binding the contract method 0x01681a62.
//
// Solidity: function sweep(address token) returns()
func (_IntegrationTests *IntegrationTestsTransactor) Sweep(opts *bind.TransactOpts, token common.Address) (*types.Transaction, error) {
	return _IntegrationTests.contract.Transact(opts, "sweep", token)
}

// Sweep is a paid mutator transaction binding the contract method 0x01681a62.
//
// Solidity: function sweep(address token) returns()
func (_IntegrationTests *IntegrationTestsSession) Sweep(token common.Address) (*types.Transaction, error) {
	return _IntegrationTests.Contract.Sweep(&_IntegrationTests.TransactOpts, token)
}

// Sweep is a paid mutator transaction binding the contract method 0x01681a62.
//
// Solidity: function sweep(address token) returns()
func (_IntegrationTests *IntegrationTestsTransactorSession) Sweep(token common.Address) (*types.Transaction, error) {
	return _IntegrationTests.Contract.Sweep(&_IntegrationTests.TransactOpts, token)
}

// TransferFees is a paid mutator transaction binding the contract method 0xc2fbe7bc.
//
// Solidity: function transferFees() returns()
func (_IntegrationTests *IntegrationTestsTransactor) TransferFees(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _IntegrationTests.contract.Transact(opts, "transferFees")
}

// TransferFees is a paid mutator transaction binding the contract method 0xc2fbe7bc.
//
// Solidity: function transferFees() returns()
func (_IntegrationTests *IntegrationTestsSession) TransferFees() (*types.Transaction, error) {
	return _IntegrationTests.Contract.TransferFees(&_IntegrationTests.TransactOpts)
}

// TransferFees is a paid mutator transaction binding the contract method 0xc2fbe7bc.
//
// Solidity: function transferFees() returns()
func (_IntegrationTests *IntegrationTestsTransactorSession) TransferFees() (*types.Transaction, error) {
	return _IntegrationTests.Contract.TransferFees(&_IntegrationTests.TransactOpts)
}

// IntegrationTestsMockAccrueFeesIterator is returned from FilterMockAccrueFees and is used to iterate over the raw logs and unpacked data for MockAccrueFees events raised by the IntegrationTests contract.
type IntegrationTestsMockAccrueFeesIterator struct {
	Event *IntegrationTestsMockAccrueFees // Event containing the contract specifics and raw log

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
func (it *IntegrationTestsMockAccrueFeesIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(IntegrationTestsMockAccrueFees)
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
		it.Event = new(IntegrationTestsMockAccrueFees)
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
func (it *IntegrationTestsMockAccrueFeesIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *IntegrationTestsMockAccrueFeesIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// IntegrationTestsMockAccrueFees represents a MockAccrueFees event raised by the IntegrationTests contract.
type IntegrationTestsMockAccrueFees struct {
	Raw types.Log // Blockchain specific contextual infos
}

// FilterMockAccrueFees is a free log retrieval operation binding the contract event 0xeea290d1596cec66f2811f6ec6d4b3e366f8f9b263e15ea87fb624ab69924042.
//
// Solidity: event mockAccrueFees()
func (_IntegrationTests *IntegrationTestsFilterer) FilterMockAccrueFees(opts *bind.FilterOpts) (*IntegrationTestsMockAccrueFeesIterator, error) {

	logs, sub, err := _IntegrationTests.contract.FilterLogs(opts, "mockAccrueFees")
	if err != nil {
		return nil, err
	}
	return &IntegrationTestsMockAccrueFeesIterator{contract: _IntegrationTests.contract, event: "mockAccrueFees", logs: logs, sub: sub}, nil
}

// WatchMockAccrueFees is a free log subscription operation binding the contract event 0xeea290d1596cec66f2811f6ec6d4b3e366f8f9b263e15ea87fb624ab69924042.
//
// Solidity: event mockAccrueFees()
func (_IntegrationTests *IntegrationTestsFilterer) WatchMockAccrueFees(opts *bind.WatchOpts, sink chan<- *IntegrationTestsMockAccrueFees) (event.Subscription, error) {

	logs, sub, err := _IntegrationTests.contract.WatchLogs(opts, "mockAccrueFees")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(IntegrationTestsMockAccrueFees)
				if err := _IntegrationTests.contract.UnpackLog(event, "mockAccrueFees", log); err != nil {
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
func (_IntegrationTests *IntegrationTestsFilterer) ParseMockAccrueFees(log types.Log) (*IntegrationTestsMockAccrueFees, error) {
	event := new(IntegrationTestsMockAccrueFees)
	if err := _IntegrationTests.contract.UnpackLog(event, "mockAccrueFees", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// IntegrationTestsMockClaimAndUnstakeIterator is returned from FilterMockClaimAndUnstake and is used to iterate over the raw logs and unpacked data for MockClaimAndUnstake events raised by the IntegrationTests contract.
type IntegrationTestsMockClaimAndUnstakeIterator struct {
	Event *IntegrationTestsMockClaimAndUnstake // Event containing the contract specifics and raw log

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
func (it *IntegrationTestsMockClaimAndUnstakeIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(IntegrationTestsMockClaimAndUnstake)
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
		it.Event = new(IntegrationTestsMockClaimAndUnstake)
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
func (it *IntegrationTestsMockClaimAndUnstakeIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *IntegrationTestsMockClaimAndUnstakeIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// IntegrationTestsMockClaimAndUnstake represents a MockClaimAndUnstake event raised by the IntegrationTests contract.
type IntegrationTestsMockClaimAndUnstake struct {
	Raw types.Log // Blockchain specific contextual infos
}

// FilterMockClaimAndUnstake is a free log retrieval operation binding the contract event 0xbfb260c8d19d48ed37a6d7379cdb9622833ea0d1f8389d3b16b0a9dc5cce17b8.
//
// Solidity: event mockClaimAndUnstake()
func (_IntegrationTests *IntegrationTestsFilterer) FilterMockClaimAndUnstake(opts *bind.FilterOpts) (*IntegrationTestsMockClaimAndUnstakeIterator, error) {

	logs, sub, err := _IntegrationTests.contract.FilterLogs(opts, "mockClaimAndUnstake")
	if err != nil {
		return nil, err
	}
	return &IntegrationTestsMockClaimAndUnstakeIterator{contract: _IntegrationTests.contract, event: "mockClaimAndUnstake", logs: logs, sub: sub}, nil
}

// WatchMockClaimAndUnstake is a free log subscription operation binding the contract event 0xbfb260c8d19d48ed37a6d7379cdb9622833ea0d1f8389d3b16b0a9dc5cce17b8.
//
// Solidity: event mockClaimAndUnstake()
func (_IntegrationTests *IntegrationTestsFilterer) WatchMockClaimAndUnstake(opts *bind.WatchOpts, sink chan<- *IntegrationTestsMockClaimAndUnstake) (event.Subscription, error) {

	logs, sub, err := _IntegrationTests.contract.WatchLogs(opts, "mockClaimAndUnstake")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(IntegrationTestsMockClaimAndUnstake)
				if err := _IntegrationTests.contract.UnpackLog(event, "mockClaimAndUnstake", log); err != nil {
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
func (_IntegrationTests *IntegrationTestsFilterer) ParseMockClaimAndUnstake(log types.Log) (*IntegrationTestsMockClaimAndUnstake, error) {
	event := new(IntegrationTestsMockClaimAndUnstake)
	if err := _IntegrationTests.contract.UnpackLog(event, "mockClaimAndUnstake", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// IntegrationTestsMockEnterStrategyIterator is returned from FilterMockEnterStrategy and is used to iterate over the raw logs and unpacked data for MockEnterStrategy events raised by the IntegrationTests contract.
type IntegrationTestsMockEnterStrategyIterator struct {
	Event *IntegrationTestsMockEnterStrategy // Event containing the contract specifics and raw log

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
func (it *IntegrationTestsMockEnterStrategyIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(IntegrationTestsMockEnterStrategy)
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
		it.Event = new(IntegrationTestsMockEnterStrategy)
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
func (it *IntegrationTestsMockEnterStrategyIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *IntegrationTestsMockEnterStrategyIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// IntegrationTestsMockEnterStrategy represents a MockEnterStrategy event raised by the IntegrationTests contract.
type IntegrationTestsMockEnterStrategy struct {
	Raw types.Log // Blockchain specific contextual infos
}

// FilterMockEnterStrategy is a free log retrieval operation binding the contract event 0x79f347e286799e0fdf15d8e5c56e5ee06f352aefddae4eb82622757f03ea240c.
//
// Solidity: event mockEnterStrategy()
func (_IntegrationTests *IntegrationTestsFilterer) FilterMockEnterStrategy(opts *bind.FilterOpts) (*IntegrationTestsMockEnterStrategyIterator, error) {

	logs, sub, err := _IntegrationTests.contract.FilterLogs(opts, "mockEnterStrategy")
	if err != nil {
		return nil, err
	}
	return &IntegrationTestsMockEnterStrategyIterator{contract: _IntegrationTests.contract, event: "mockEnterStrategy", logs: logs, sub: sub}, nil
}

// WatchMockEnterStrategy is a free log subscription operation binding the contract event 0x79f347e286799e0fdf15d8e5c56e5ee06f352aefddae4eb82622757f03ea240c.
//
// Solidity: event mockEnterStrategy()
func (_IntegrationTests *IntegrationTestsFilterer) WatchMockEnterStrategy(opts *bind.WatchOpts, sink chan<- *IntegrationTestsMockEnterStrategy) (event.Subscription, error) {

	logs, sub, err := _IntegrationTests.contract.WatchLogs(opts, "mockEnterStrategy")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(IntegrationTestsMockEnterStrategy)
				if err := _IntegrationTests.contract.UnpackLog(event, "mockEnterStrategy", log); err != nil {
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
func (_IntegrationTests *IntegrationTestsFilterer) ParseMockEnterStrategy(log types.Log) (*IntegrationTestsMockEnterStrategy, error) {
	event := new(IntegrationTestsMockEnterStrategy)
	if err := _IntegrationTests.contract.UnpackLog(event, "mockEnterStrategy", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// IntegrationTestsMockPauseIterator is returned from FilterMockPause and is used to iterate over the raw logs and unpacked data for MockPause events raised by the IntegrationTests contract.
type IntegrationTestsMockPauseIterator struct {
	Event *IntegrationTestsMockPause // Event containing the contract specifics and raw log

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
func (it *IntegrationTestsMockPauseIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(IntegrationTestsMockPause)
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
		it.Event = new(IntegrationTestsMockPause)
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
func (it *IntegrationTestsMockPauseIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *IntegrationTestsMockPauseIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// IntegrationTestsMockPause represents a MockPause event raised by the IntegrationTests contract.
type IntegrationTestsMockPause struct {
	IsPaused bool
	Raw      types.Log // Blockchain specific contextual infos
}

// FilterMockPause is a free log retrieval operation binding the contract event 0xb24266e8968c43c3fcdc4ee6e69d484c1a6bc4fd4d00b54e1d5c75fec4896210.
//
// Solidity: event mockPause(bool isPaused)
func (_IntegrationTests *IntegrationTestsFilterer) FilterMockPause(opts *bind.FilterOpts) (*IntegrationTestsMockPauseIterator, error) {

	logs, sub, err := _IntegrationTests.contract.FilterLogs(opts, "mockPause")
	if err != nil {
		return nil, err
	}
	return &IntegrationTestsMockPauseIterator{contract: _IntegrationTests.contract, event: "mockPause", logs: logs, sub: sub}, nil
}

// WatchMockPause is a free log subscription operation binding the contract event 0xb24266e8968c43c3fcdc4ee6e69d484c1a6bc4fd4d00b54e1d5c75fec4896210.
//
// Solidity: event mockPause(bool isPaused)
func (_IntegrationTests *IntegrationTestsFilterer) WatchMockPause(opts *bind.WatchOpts, sink chan<- *IntegrationTestsMockPause) (event.Subscription, error) {

	logs, sub, err := _IntegrationTests.contract.WatchLogs(opts, "mockPause")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(IntegrationTestsMockPause)
				if err := _IntegrationTests.contract.UnpackLog(event, "mockPause", log); err != nil {
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
func (_IntegrationTests *IntegrationTestsFilterer) ParseMockPause(log types.Log) (*IntegrationTestsMockPause, error) {
	event := new(IntegrationTestsMockPause)
	if err := _IntegrationTests.contract.UnpackLog(event, "mockPause", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// IntegrationTestsMockRebalanceIterator is returned from FilterMockRebalance and is used to iterate over the raw logs and unpacked data for MockRebalance events raised by the IntegrationTests contract.
type IntegrationTestsMockRebalanceIterator struct {
	Event *IntegrationTestsMockRebalance // Event containing the contract specifics and raw log

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
func (it *IntegrationTestsMockRebalanceIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(IntegrationTestsMockRebalance)
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
		it.Event = new(IntegrationTestsMockRebalance)
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
func (it *IntegrationTestsMockRebalanceIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *IntegrationTestsMockRebalanceIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// IntegrationTestsMockRebalance represents a MockRebalance event raised by the IntegrationTests contract.
type IntegrationTestsMockRebalance struct {
	Path             []common.Address
	AmountOutMinimum *big.Int
	Raw              types.Log // Blockchain specific contextual infos
}

// FilterMockRebalance is a free log retrieval operation binding the contract event 0x25242bda07a46aa7e9d72c7aca42b90b6883a257ed876327d393c2a1c832eef1.
//
// Solidity: event mockRebalance(address[] path, uint256 amountOutMinimum)
func (_IntegrationTests *IntegrationTestsFilterer) FilterMockRebalance(opts *bind.FilterOpts) (*IntegrationTestsMockRebalanceIterator, error) {

	logs, sub, err := _IntegrationTests.contract.FilterLogs(opts, "mockRebalance")
	if err != nil {
		return nil, err
	}
	return &IntegrationTestsMockRebalanceIterator{contract: _IntegrationTests.contract, event: "mockRebalance", logs: logs, sub: sub}, nil
}

// WatchMockRebalance is a free log subscription operation binding the contract event 0x25242bda07a46aa7e9d72c7aca42b90b6883a257ed876327d393c2a1c832eef1.
//
// Solidity: event mockRebalance(address[] path, uint256 amountOutMinimum)
func (_IntegrationTests *IntegrationTestsFilterer) WatchMockRebalance(opts *bind.WatchOpts, sink chan<- *IntegrationTestsMockRebalance) (event.Subscription, error) {

	logs, sub, err := _IntegrationTests.contract.WatchLogs(opts, "mockRebalance")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(IntegrationTestsMockRebalance)
				if err := _IntegrationTests.contract.UnpackLog(event, "mockRebalance", log); err != nil {
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
func (_IntegrationTests *IntegrationTestsFilterer) ParseMockRebalance(log types.Log) (*IntegrationTestsMockRebalance, error) {
	event := new(IntegrationTestsMockRebalance)
	if err := _IntegrationTests.contract.UnpackLog(event, "mockRebalance", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// IntegrationTestsMockReinvestIterator is returned from FilterMockReinvest and is used to iterate over the raw logs and unpacked data for MockReinvest events raised by the IntegrationTests contract.
type IntegrationTestsMockReinvestIterator struct {
	Event *IntegrationTestsMockReinvest // Event containing the contract specifics and raw log

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
func (it *IntegrationTestsMockReinvestIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(IntegrationTestsMockReinvest)
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
		it.Event = new(IntegrationTestsMockReinvest)
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
func (it *IntegrationTestsMockReinvestIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *IntegrationTestsMockReinvestIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// IntegrationTestsMockReinvest represents a MockReinvest event raised by the IntegrationTests contract.
type IntegrationTestsMockReinvest struct {
	Path         []common.Address
	MinAssetsOut *big.Int
	Raw          types.Log // Blockchain specific contextual infos
}

// FilterMockReinvest is a free log retrieval operation binding the contract event 0xa820edc96d666f54cd527363abf610e586001232cbb3a97a3c61f121222032d0.
//
// Solidity: event mockReinvest(address[] path, uint256 minAssetsOut)
func (_IntegrationTests *IntegrationTestsFilterer) FilterMockReinvest(opts *bind.FilterOpts) (*IntegrationTestsMockReinvestIterator, error) {

	logs, sub, err := _IntegrationTests.contract.FilterLogs(opts, "mockReinvest")
	if err != nil {
		return nil, err
	}
	return &IntegrationTestsMockReinvestIterator{contract: _IntegrationTests.contract, event: "mockReinvest", logs: logs, sub: sub}, nil
}

// WatchMockReinvest is a free log subscription operation binding the contract event 0xa820edc96d666f54cd527363abf610e586001232cbb3a97a3c61f121222032d0.
//
// Solidity: event mockReinvest(address[] path, uint256 minAssetsOut)
func (_IntegrationTests *IntegrationTestsFilterer) WatchMockReinvest(opts *bind.WatchOpts, sink chan<- *IntegrationTestsMockReinvest) (event.Subscription, error) {

	logs, sub, err := _IntegrationTests.contract.WatchLogs(opts, "mockReinvest")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(IntegrationTestsMockReinvest)
				if err := _IntegrationTests.contract.UnpackLog(event, "mockReinvest", log); err != nil {
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
func (_IntegrationTests *IntegrationTestsFilterer) ParseMockReinvest(log types.Log) (*IntegrationTestsMockReinvest, error) {
	event := new(IntegrationTestsMockReinvest)
	if err := _IntegrationTests.contract.UnpackLog(event, "mockReinvest", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// IntegrationTestsMockRemoveLiquidityRestrictionIterator is returned from FilterMockRemoveLiquidityRestriction and is used to iterate over the raw logs and unpacked data for MockRemoveLiquidityRestriction events raised by the IntegrationTests contract.
type IntegrationTestsMockRemoveLiquidityRestrictionIterator struct {
	Event *IntegrationTestsMockRemoveLiquidityRestriction // Event containing the contract specifics and raw log

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
func (it *IntegrationTestsMockRemoveLiquidityRestrictionIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(IntegrationTestsMockRemoveLiquidityRestriction)
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
		it.Event = new(IntegrationTestsMockRemoveLiquidityRestriction)
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
func (it *IntegrationTestsMockRemoveLiquidityRestrictionIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *IntegrationTestsMockRemoveLiquidityRestrictionIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// IntegrationTestsMockRemoveLiquidityRestriction represents a MockRemoveLiquidityRestriction event raised by the IntegrationTests contract.
type IntegrationTestsMockRemoveLiquidityRestriction struct {
	Raw types.Log // Blockchain specific contextual infos
}

// FilterMockRemoveLiquidityRestriction is a free log retrieval operation binding the contract event 0x357aa2f9a52ebf8e95a37ec4381ec941f12211d04a5874c78159ebcc3d58aa7e.
//
// Solidity: event mockRemoveLiquidityRestriction()
func (_IntegrationTests *IntegrationTestsFilterer) FilterMockRemoveLiquidityRestriction(opts *bind.FilterOpts) (*IntegrationTestsMockRemoveLiquidityRestrictionIterator, error) {

	logs, sub, err := _IntegrationTests.contract.FilterLogs(opts, "mockRemoveLiquidityRestriction")
	if err != nil {
		return nil, err
	}
	return &IntegrationTestsMockRemoveLiquidityRestrictionIterator{contract: _IntegrationTests.contract, event: "mockRemoveLiquidityRestriction", logs: logs, sub: sub}, nil
}

// WatchMockRemoveLiquidityRestriction is a free log subscription operation binding the contract event 0x357aa2f9a52ebf8e95a37ec4381ec941f12211d04a5874c78159ebcc3d58aa7e.
//
// Solidity: event mockRemoveLiquidityRestriction()
func (_IntegrationTests *IntegrationTestsFilterer) WatchMockRemoveLiquidityRestriction(opts *bind.WatchOpts, sink chan<- *IntegrationTestsMockRemoveLiquidityRestriction) (event.Subscription, error) {

	logs, sub, err := _IntegrationTests.contract.WatchLogs(opts, "mockRemoveLiquidityRestriction")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(IntegrationTestsMockRemoveLiquidityRestriction)
				if err := _IntegrationTests.contract.UnpackLog(event, "mockRemoveLiquidityRestriction", log); err != nil {
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
func (_IntegrationTests *IntegrationTestsFilterer) ParseMockRemoveLiquidityRestriction(log types.Log) (*IntegrationTestsMockRemoveLiquidityRestriction, error) {
	event := new(IntegrationTestsMockRemoveLiquidityRestriction)
	if err := _IntegrationTests.contract.UnpackLog(event, "mockRemoveLiquidityRestriction", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// IntegrationTestsMockShutdownIterator is returned from FilterMockShutdown and is used to iterate over the raw logs and unpacked data for MockShutdown events raised by the IntegrationTests contract.
type IntegrationTestsMockShutdownIterator struct {
	Event *IntegrationTestsMockShutdown // Event containing the contract specifics and raw log

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
func (it *IntegrationTestsMockShutdownIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(IntegrationTestsMockShutdown)
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
		it.Event = new(IntegrationTestsMockShutdown)
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
func (it *IntegrationTestsMockShutdownIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *IntegrationTestsMockShutdownIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// IntegrationTestsMockShutdown represents a MockShutdown event raised by the IntegrationTests contract.
type IntegrationTestsMockShutdown struct {
	Raw types.Log // Blockchain specific contextual infos
}

// FilterMockShutdown is a free log retrieval operation binding the contract event 0xf02bc92cc12edf99f533f386e56b3945a3e30921d9f866f5b291b886b0fe0632.
//
// Solidity: event mockShutdown()
func (_IntegrationTests *IntegrationTestsFilterer) FilterMockShutdown(opts *bind.FilterOpts) (*IntegrationTestsMockShutdownIterator, error) {

	logs, sub, err := _IntegrationTests.contract.FilterLogs(opts, "mockShutdown")
	if err != nil {
		return nil, err
	}
	return &IntegrationTestsMockShutdownIterator{contract: _IntegrationTests.contract, event: "mockShutdown", logs: logs, sub: sub}, nil
}

// WatchMockShutdown is a free log subscription operation binding the contract event 0xf02bc92cc12edf99f533f386e56b3945a3e30921d9f866f5b291b886b0fe0632.
//
// Solidity: event mockShutdown()
func (_IntegrationTests *IntegrationTestsFilterer) WatchMockShutdown(opts *bind.WatchOpts, sink chan<- *IntegrationTestsMockShutdown) (event.Subscription, error) {

	logs, sub, err := _IntegrationTests.contract.WatchLogs(opts, "mockShutdown")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(IntegrationTestsMockShutdown)
				if err := _IntegrationTests.contract.UnpackLog(event, "mockShutdown", log); err != nil {
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
func (_IntegrationTests *IntegrationTestsFilterer) ParseMockShutdown(log types.Log) (*IntegrationTestsMockShutdown, error) {
	event := new(IntegrationTestsMockShutdown)
	if err := _IntegrationTests.contract.UnpackLog(event, "mockShutdown", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// IntegrationTestsMockSweepIterator is returned from FilterMockSweep and is used to iterate over the raw logs and unpacked data for MockSweep events raised by the IntegrationTests contract.
type IntegrationTestsMockSweepIterator struct {
	Event *IntegrationTestsMockSweep // Event containing the contract specifics and raw log

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
func (it *IntegrationTestsMockSweepIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(IntegrationTestsMockSweep)
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
		it.Event = new(IntegrationTestsMockSweep)
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
func (it *IntegrationTestsMockSweepIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *IntegrationTestsMockSweepIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// IntegrationTestsMockSweep represents a MockSweep event raised by the IntegrationTests contract.
type IntegrationTestsMockSweep struct {
	Token common.Address
	Raw   types.Log // Blockchain specific contextual infos
}

// FilterMockSweep is a free log retrieval operation binding the contract event 0xc352904aa12ccc4e1b1ae66914c29a0b6aa68f6554ab53b0d3943fabe742d78b.
//
// Solidity: event mockSweep(address token)
func (_IntegrationTests *IntegrationTestsFilterer) FilterMockSweep(opts *bind.FilterOpts) (*IntegrationTestsMockSweepIterator, error) {

	logs, sub, err := _IntegrationTests.contract.FilterLogs(opts, "mockSweep")
	if err != nil {
		return nil, err
	}
	return &IntegrationTestsMockSweepIterator{contract: _IntegrationTests.contract, event: "mockSweep", logs: logs, sub: sub}, nil
}

// WatchMockSweep is a free log subscription operation binding the contract event 0xc352904aa12ccc4e1b1ae66914c29a0b6aa68f6554ab53b0d3943fabe742d78b.
//
// Solidity: event mockSweep(address token)
func (_IntegrationTests *IntegrationTestsFilterer) WatchMockSweep(opts *bind.WatchOpts, sink chan<- *IntegrationTestsMockSweep) (event.Subscription, error) {

	logs, sub, err := _IntegrationTests.contract.WatchLogs(opts, "mockSweep")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(IntegrationTestsMockSweep)
				if err := _IntegrationTests.contract.UnpackLog(event, "mockSweep", log); err != nil {
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
func (_IntegrationTests *IntegrationTestsFilterer) ParseMockSweep(log types.Log) (*IntegrationTestsMockSweep, error) {
	event := new(IntegrationTestsMockSweep)
	if err := _IntegrationTests.contract.UnpackLog(event, "mockSweep", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// IntegrationTestsMockTransferFeesIterator is returned from FilterMockTransferFees and is used to iterate over the raw logs and unpacked data for MockTransferFees events raised by the IntegrationTests contract.
type IntegrationTestsMockTransferFeesIterator struct {
	Event *IntegrationTestsMockTransferFees // Event containing the contract specifics and raw log

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
func (it *IntegrationTestsMockTransferFeesIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(IntegrationTestsMockTransferFees)
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
		it.Event = new(IntegrationTestsMockTransferFees)
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
func (it *IntegrationTestsMockTransferFeesIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *IntegrationTestsMockTransferFeesIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// IntegrationTestsMockTransferFees represents a MockTransferFees event raised by the IntegrationTests contract.
type IntegrationTestsMockTransferFees struct {
	Raw types.Log // Blockchain specific contextual infos
}

// FilterMockTransferFees is a free log retrieval operation binding the contract event 0x9e1dd58aa34e20b52e0befa7246182b86d06fd56ea0a521d4d3dc8d6241cc43a.
//
// Solidity: event mockTransferFees()
func (_IntegrationTests *IntegrationTestsFilterer) FilterMockTransferFees(opts *bind.FilterOpts) (*IntegrationTestsMockTransferFeesIterator, error) {

	logs, sub, err := _IntegrationTests.contract.FilterLogs(opts, "mockTransferFees")
	if err != nil {
		return nil, err
	}
	return &IntegrationTestsMockTransferFeesIterator{contract: _IntegrationTests.contract, event: "mockTransferFees", logs: logs, sub: sub}, nil
}

// WatchMockTransferFees is a free log subscription operation binding the contract event 0x9e1dd58aa34e20b52e0befa7246182b86d06fd56ea0a521d4d3dc8d6241cc43a.
//
// Solidity: event mockTransferFees()
func (_IntegrationTests *IntegrationTestsFilterer) WatchMockTransferFees(opts *bind.WatchOpts, sink chan<- *IntegrationTestsMockTransferFees) (event.Subscription, error) {

	logs, sub, err := _IntegrationTests.contract.WatchLogs(opts, "mockTransferFees")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(IntegrationTestsMockTransferFees)
				if err := _IntegrationTests.contract.UnpackLog(event, "mockTransferFees", log); err != nil {
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
func (_IntegrationTests *IntegrationTestsFilterer) ParseMockTransferFees(log types.Log) (*IntegrationTestsMockTransferFees, error) {
	event := new(IntegrationTestsMockTransferFees)
	if err := _IntegrationTests.contract.UnpackLog(event, "mockTransferFees", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}
