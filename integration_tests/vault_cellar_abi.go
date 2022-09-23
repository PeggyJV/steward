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

// CellarMetaData contains all meta data concerning the Cellar contract.
var CellarMetaData = &bind.MetaData{
	ABI: "[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"fromPosition\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"toPosition\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"assetsFrom\",\"type\":\"uint256\"}],\"name\":\"Rebalance\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"fromPosition\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"toPosition\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"assetsFrom\",\"type\":\"uint256\"},{\"internalType\":\"enumCellar.Exchange\",\"name\":\"exchange\",\"type\":\"uint8\"},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\"}],\"name\":\"rebalance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"assetsTo\",\"type\":\"uint256\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]",
}

// CellarABI is the input ABI used to generate the binding from.
// Deprecated: Use CellarMetaData.ABI instead.
var CellarABI = CellarMetaData.ABI

// Cellar is an auto generated Go binding around an Ethereum contract.
type Cellar struct {
	CellarCaller     // Read-only binding to the contract
	CellarTransactor // Write-only binding to the contract
	CellarFilterer   // Log filterer for contract events
}

// CellarCaller is an auto generated read-only Go binding around an Ethereum contract.
type CellarCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// CellarTransactor is an auto generated write-only Go binding around an Ethereum contract.
type CellarTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// CellarFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type CellarFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// CellarSession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type CellarSession struct {
	Contract     *Cellar           // Generic contract binding to set the session for
	CallOpts     bind.CallOpts     // Call options to use throughout this session
	TransactOpts bind.TransactOpts // Transaction auth options to use throughout this session
}

// CellarCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type CellarCallerSession struct {
	Contract *CellarCaller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts // Call options to use throughout this session
}

// CellarTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type CellarTransactorSession struct {
	Contract     *CellarTransactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts // Transaction auth options to use throughout this session
}

// CellarRaw is an auto generated low-level Go binding around an Ethereum contract.
type CellarRaw struct {
	Contract *Cellar // Generic contract binding to access the raw methods on
}

// CellarCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type CellarCallerRaw struct {
	Contract *CellarCaller // Generic read-only contract binding to access the raw methods on
}

// CellarTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type CellarTransactorRaw struct {
	Contract *CellarTransactor // Generic write-only contract binding to access the raw methods on
}

// NewCellar creates a new instance of Cellar, bound to a specific deployed contract.
func NewCellar(address common.Address, backend bind.ContractBackend) (*Cellar, error) {
	contract, err := bindCellar(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &Cellar{CellarCaller: CellarCaller{contract: contract}, CellarTransactor: CellarTransactor{contract: contract}, CellarFilterer: CellarFilterer{contract: contract}}, nil
}

// NewCellarCaller creates a new read-only instance of Cellar, bound to a specific deployed contract.
func NewCellarCaller(address common.Address, caller bind.ContractCaller) (*CellarCaller, error) {
	contract, err := bindCellar(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &CellarCaller{contract: contract}, nil
}

// NewCellarTransactor creates a new write-only instance of Cellar, bound to a specific deployed contract.
func NewCellarTransactor(address common.Address, transactor bind.ContractTransactor) (*CellarTransactor, error) {
	contract, err := bindCellar(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &CellarTransactor{contract: contract}, nil
}

// NewCellarFilterer creates a new log filterer instance of Cellar, bound to a specific deployed contract.
func NewCellarFilterer(address common.Address, filterer bind.ContractFilterer) (*CellarFilterer, error) {
	contract, err := bindCellar(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &CellarFilterer{contract: contract}, nil
}

// bindCellar binds a generic wrapper to an already deployed contract.
func bindCellar(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := abi.JSON(strings.NewReader(CellarABI))
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_Cellar *CellarRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _Cellar.Contract.CellarCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_Cellar *CellarRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _Cellar.Contract.CellarTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_Cellar *CellarRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _Cellar.Contract.CellarTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_Cellar *CellarCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _Cellar.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_Cellar *CellarTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _Cellar.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_Cellar *CellarTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _Cellar.Contract.contract.Transact(opts, method, params...)
}

// Rebalance is a paid mutator transaction binding the contract method 0x389a7294.
//
// Solidity: function rebalance(address fromPosition, address toPosition, uint256 assetsFrom, uint8 exchange, bytes params) returns(uint256 assetsTo)
func (_Cellar *CellarTransactor) Rebalance(opts *bind.TransactOpts, fromPosition common.Address, toPosition common.Address, assetsFrom *big.Int, exchange uint8, params []byte) (*types.Transaction, error) {
	return _Cellar.contract.Transact(opts, "rebalance", fromPosition, toPosition, assetsFrom, exchange, params)
}

// Rebalance is a paid mutator transaction binding the contract method 0x389a7294.
//
// Solidity: function rebalance(address fromPosition, address toPosition, uint256 assetsFrom, uint8 exchange, bytes params) returns(uint256 assetsTo)
func (_Cellar *CellarSession) Rebalance(fromPosition common.Address, toPosition common.Address, assetsFrom *big.Int, exchange uint8, params []byte) (*types.Transaction, error) {
	return _Cellar.Contract.Rebalance(&_Cellar.TransactOpts, fromPosition, toPosition, assetsFrom, exchange, params)
}

// Rebalance is a paid mutator transaction binding the contract method 0x389a7294.
//
// Solidity: function rebalance(address fromPosition, address toPosition, uint256 assetsFrom, uint8 exchange, bytes params) returns(uint256 assetsTo)
func (_Cellar *CellarTransactorSession) Rebalance(fromPosition common.Address, toPosition common.Address, assetsFrom *big.Int, exchange uint8, params []byte) (*types.Transaction, error) {
	return _Cellar.Contract.Rebalance(&_Cellar.TransactOpts, fromPosition, toPosition, assetsFrom, exchange, params)
}

// CellarRebalanceIterator is returned from FilterRebalance and is used to iterate over the raw logs and unpacked data for Rebalance events raised by the Cellar contract.
type CellarRebalanceIterator struct {
	Event *CellarRebalance // Event containing the contract specifics and raw log

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
func (it *CellarRebalanceIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(CellarRebalance)
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
		it.Event = new(CellarRebalance)
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
func (it *CellarRebalanceIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *CellarRebalanceIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// CellarRebalance represents a Rebalance event raised by the Cellar contract.
type CellarRebalance struct {
	FromPosition common.Address
	ToPosition   common.Address
	AssetsFrom   *big.Int
	Raw          types.Log // Blockchain specific contextual infos
}

// FilterRebalance is a free log retrieval operation binding the contract event 0xb0850b8e0f9e8315dde3c9f9f31138283e6bbe16cd29e8552eb1dcdf9fac9e3b.
//
// Solidity: event Rebalance(address indexed fromPosition, address indexed toPosition, uint256 assetsFrom)
func (_Cellar *CellarFilterer) FilterRebalance(opts *bind.FilterOpts, fromPosition []common.Address, toPosition []common.Address) (*CellarRebalanceIterator, error) {

	var fromPositionRule []interface{}
	for _, fromPositionItem := range fromPosition {
		fromPositionRule = append(fromPositionRule, fromPositionItem)
	}
	var toPositionRule []interface{}
	for _, toPositionItem := range toPosition {
		toPositionRule = append(toPositionRule, toPositionItem)
	}

	logs, sub, err := _Cellar.contract.FilterLogs(opts, "Rebalance", fromPositionRule, toPositionRule)
	if err != nil {
		return nil, err
	}
	return &CellarRebalanceIterator{contract: _Cellar.contract, event: "Rebalance", logs: logs, sub: sub}, nil
}

// WatchRebalance is a free log subscription operation binding the contract event 0xb0850b8e0f9e8315dde3c9f9f31138283e6bbe16cd29e8552eb1dcdf9fac9e3b.
//
// Solidity: event Rebalance(address indexed fromPosition, address indexed toPosition, uint256 assetsFrom)
func (_Cellar *CellarFilterer) WatchRebalance(opts *bind.WatchOpts, sink chan<- *CellarRebalance, fromPosition []common.Address, toPosition []common.Address) (event.Subscription, error) {

	var fromPositionRule []interface{}
	for _, fromPositionItem := range fromPosition {
		fromPositionRule = append(fromPositionRule, fromPositionItem)
	}
	var toPositionRule []interface{}
	for _, toPositionItem := range toPosition {
		toPositionRule = append(toPositionRule, toPositionItem)
	}

	logs, sub, err := _Cellar.contract.WatchLogs(opts, "Rebalance", fromPositionRule, toPositionRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(CellarRebalance)
				if err := _Cellar.contract.UnpackLog(event, "Rebalance", log); err != nil {
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

// ParseRebalance is a log parse operation binding the contract event 0xb0850b8e0f9e8315dde3c9f9f31138283e6bbe16cd29e8552eb1dcdf9fac9e3b.
//
// Solidity: event Rebalance(address indexed fromPosition, address indexed toPosition, uint256 assetsFrom)
func (_Cellar *CellarFilterer) ParseRebalance(log types.Log) (*CellarRebalance, error) {
	event := new(CellarRebalance)
	if err := _Cellar.contract.UnpackLog(event, "Rebalance", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}
