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

// AdaptorMetaData contains all meta data concerning the Adaptor contract.
var AdaptorMetaData = &bind.MetaData{
	ABI: "[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"debtTokenToBorrow\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amountToBorrow\",\"type\":\"uint256\"}],\"name\":\"BorrowFromAave\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"contractERC20\",\"name\":\"assetOut\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"enumCellar.Exchange\",\"name\":\"exchange\",\"type\":\"uint8\"},{\"indexed\":false,\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\"},{\"indexed\":false,\"internalType\":\"uint64\",\"name\":\"slippage\",\"type\":\"uint64\"}],\"name\":\"ClaimCompAndSwap\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"OwnerUpdated\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"contractERC20\",\"name\":\"tokenIn\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"contractERC20\",\"name\":\"tokenToRepay\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"enumCellar.Exchange\",\"name\":\"exchange\",\"type\":\"uint8\"},{\"indexed\":false,\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\"}],\"name\":\"SwapAndRepay\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"debtTokenToBorrow\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amountToBorrow\",\"type\":\"uint256\"}],\"name\":\"borrowFromAave\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"contractERC20\",\"name\":\"assetOut\",\"type\":\"address\"},{\"internalType\":\"enumCellar.Exchange\",\"name\":\"exchange\",\"type\":\"uint8\"},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\"},{\"internalType\":\"uint64\",\"name\":\"slippage\",\"type\":\"uint64\"}],\"name\":\"claimCompAndSwap\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"setOwner\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"contractERC20\",\"name\":\"tokenIn\",\"type\":\"address\"},{\"internalType\":\"contractERC20\",\"name\":\"tokenToRepay\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\"},{\"internalType\":\"enumCellar.Exchange\",\"name\":\"exchange\",\"type\":\"uint8\"},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\"}],\"name\":\"swapAndRepay\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]",
}

// AdaptorABI is the input ABI used to generate the binding from.
// Deprecated: Use AdaptorMetaData.ABI instead.
var AdaptorABI = AdaptorMetaData.ABI

// Adaptor is an auto generated Go binding around an Ethereum contract.
type Adaptor struct {
	AdaptorCaller     // Read-only binding to the contract
	AdaptorTransactor // Write-only binding to the contract
	AdaptorFilterer   // Log filterer for contract events
}

// AdaptorCaller is an auto generated read-only Go binding around an Ethereum contract.
type AdaptorCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// AdaptorTransactor is an auto generated write-only Go binding around an Ethereum contract.
type AdaptorTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// AdaptorFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type AdaptorFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// AdaptorSession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type AdaptorSession struct {
	Contract     *Adaptor          // Generic contract binding to set the session for
	CallOpts     bind.CallOpts     // Call options to use throughout this session
	TransactOpts bind.TransactOpts // Transaction auth options to use throughout this session
}

// AdaptorCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type AdaptorCallerSession struct {
	Contract *AdaptorCaller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts  // Call options to use throughout this session
}

// AdaptorTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type AdaptorTransactorSession struct {
	Contract     *AdaptorTransactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts  // Transaction auth options to use throughout this session
}

// AdaptorRaw is an auto generated low-level Go binding around an Ethereum contract.
type AdaptorRaw struct {
	Contract *Adaptor // Generic contract binding to access the raw methods on
}

// AdaptorCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type AdaptorCallerRaw struct {
	Contract *AdaptorCaller // Generic read-only contract binding to access the raw methods on
}

// AdaptorTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type AdaptorTransactorRaw struct {
	Contract *AdaptorTransactor // Generic write-only contract binding to access the raw methods on
}

// NewAdaptor creates a new instance of Adaptor, bound to a specific deployed contract.
func NewAdaptor(address common.Address, backend bind.ContractBackend) (*Adaptor, error) {
	contract, err := bindAdaptor(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &Adaptor{AdaptorCaller: AdaptorCaller{contract: contract}, AdaptorTransactor: AdaptorTransactor{contract: contract}, AdaptorFilterer: AdaptorFilterer{contract: contract}}, nil
}

// NewAdaptorCaller creates a new read-only instance of Adaptor, bound to a specific deployed contract.
func NewAdaptorCaller(address common.Address, caller bind.ContractCaller) (*AdaptorCaller, error) {
	contract, err := bindAdaptor(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &AdaptorCaller{contract: contract}, nil
}

// NewAdaptorTransactor creates a new write-only instance of Adaptor, bound to a specific deployed contract.
func NewAdaptorTransactor(address common.Address, transactor bind.ContractTransactor) (*AdaptorTransactor, error) {
	contract, err := bindAdaptor(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &AdaptorTransactor{contract: contract}, nil
}

// NewAdaptorFilterer creates a new log filterer instance of Adaptor, bound to a specific deployed contract.
func NewAdaptorFilterer(address common.Address, filterer bind.ContractFilterer) (*AdaptorFilterer, error) {
	contract, err := bindAdaptor(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &AdaptorFilterer{contract: contract}, nil
}

// bindAdaptor binds a generic wrapper to an already deployed contract.
func bindAdaptor(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := abi.JSON(strings.NewReader(AdaptorABI))
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_Adaptor *AdaptorRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _Adaptor.Contract.AdaptorCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_Adaptor *AdaptorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _Adaptor.Contract.AdaptorTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_Adaptor *AdaptorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _Adaptor.Contract.AdaptorTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_Adaptor *AdaptorCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _Adaptor.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_Adaptor *AdaptorTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _Adaptor.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_Adaptor *AdaptorTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _Adaptor.Contract.contract.Transact(opts, method, params...)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_Adaptor *AdaptorCaller) Owner(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _Adaptor.contract.Call(opts, &out, "owner")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_Adaptor *AdaptorSession) Owner() (common.Address, error) {
	return _Adaptor.Contract.Owner(&_Adaptor.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_Adaptor *AdaptorCallerSession) Owner() (common.Address, error) {
	return _Adaptor.Contract.Owner(&_Adaptor.CallOpts)
}

// BorrowFromAave is a paid mutator transaction binding the contract method 0x9a85e1d3.
//
// Solidity: function borrowFromAave(address debtTokenToBorrow, uint256 amountToBorrow) returns()
func (_Adaptor *AdaptorTransactor) BorrowFromAave(opts *bind.TransactOpts, debtTokenToBorrow common.Address, amountToBorrow *big.Int) (*types.Transaction, error) {
	return _Adaptor.contract.Transact(opts, "borrowFromAave", debtTokenToBorrow, amountToBorrow)
}

// BorrowFromAave is a paid mutator transaction binding the contract method 0x9a85e1d3.
//
// Solidity: function borrowFromAave(address debtTokenToBorrow, uint256 amountToBorrow) returns()
func (_Adaptor *AdaptorSession) BorrowFromAave(debtTokenToBorrow common.Address, amountToBorrow *big.Int) (*types.Transaction, error) {
	return _Adaptor.Contract.BorrowFromAave(&_Adaptor.TransactOpts, debtTokenToBorrow, amountToBorrow)
}

// BorrowFromAave is a paid mutator transaction binding the contract method 0x9a85e1d3.
//
// Solidity: function borrowFromAave(address debtTokenToBorrow, uint256 amountToBorrow) returns()
func (_Adaptor *AdaptorTransactorSession) BorrowFromAave(debtTokenToBorrow common.Address, amountToBorrow *big.Int) (*types.Transaction, error) {
	return _Adaptor.Contract.BorrowFromAave(&_Adaptor.TransactOpts, debtTokenToBorrow, amountToBorrow)
}

// ClaimCompAndSwap is a paid mutator transaction binding the contract method 0x39e72ba8.
//
// Solidity: function claimCompAndSwap(address assetOut, uint8 exchange, bytes params, uint64 slippage) returns()
func (_Adaptor *AdaptorTransactor) ClaimCompAndSwap(opts *bind.TransactOpts, assetOut common.Address, exchange uint8, params []byte, slippage uint64) (*types.Transaction, error) {
	return _Adaptor.contract.Transact(opts, "claimCompAndSwap", assetOut, exchange, params, slippage)
}

// ClaimCompAndSwap is a paid mutator transaction binding the contract method 0x39e72ba8.
//
// Solidity: function claimCompAndSwap(address assetOut, uint8 exchange, bytes params, uint64 slippage) returns()
func (_Adaptor *AdaptorSession) ClaimCompAndSwap(assetOut common.Address, exchange uint8, params []byte, slippage uint64) (*types.Transaction, error) {
	return _Adaptor.Contract.ClaimCompAndSwap(&_Adaptor.TransactOpts, assetOut, exchange, params, slippage)
}

// ClaimCompAndSwap is a paid mutator transaction binding the contract method 0x39e72ba8.
//
// Solidity: function claimCompAndSwap(address assetOut, uint8 exchange, bytes params, uint64 slippage) returns()
func (_Adaptor *AdaptorTransactorSession) ClaimCompAndSwap(assetOut common.Address, exchange uint8, params []byte, slippage uint64) (*types.Transaction, error) {
	return _Adaptor.Contract.ClaimCompAndSwap(&_Adaptor.TransactOpts, assetOut, exchange, params, slippage)
}

// SetOwner is a paid mutator transaction binding the contract method 0x13af4035.
//
// Solidity: function setOwner(address newOwner) returns()
func (_Adaptor *AdaptorTransactor) SetOwner(opts *bind.TransactOpts, newOwner common.Address) (*types.Transaction, error) {
	return _Adaptor.contract.Transact(opts, "setOwner", newOwner)
}

// SetOwner is a paid mutator transaction binding the contract method 0x13af4035.
//
// Solidity: function setOwner(address newOwner) returns()
func (_Adaptor *AdaptorSession) SetOwner(newOwner common.Address) (*types.Transaction, error) {
	return _Adaptor.Contract.SetOwner(&_Adaptor.TransactOpts, newOwner)
}

// SetOwner is a paid mutator transaction binding the contract method 0x13af4035.
//
// Solidity: function setOwner(address newOwner) returns()
func (_Adaptor *AdaptorTransactorSession) SetOwner(newOwner common.Address) (*types.Transaction, error) {
	return _Adaptor.Contract.SetOwner(&_Adaptor.TransactOpts, newOwner)
}

// SwapAndRepay is a paid mutator transaction binding the contract method 0x642c2d8e.
//
// Solidity: function swapAndRepay(address tokenIn, address tokenToRepay, uint256 amountIn, uint8 exchange, bytes params) returns()
func (_Adaptor *AdaptorTransactor) SwapAndRepay(opts *bind.TransactOpts, tokenIn common.Address, tokenToRepay common.Address, amountIn *big.Int, exchange uint8, params []byte) (*types.Transaction, error) {
	return _Adaptor.contract.Transact(opts, "swapAndRepay", tokenIn, tokenToRepay, amountIn, exchange, params)
}

// SwapAndRepay is a paid mutator transaction binding the contract method 0x642c2d8e.
//
// Solidity: function swapAndRepay(address tokenIn, address tokenToRepay, uint256 amountIn, uint8 exchange, bytes params) returns()
func (_Adaptor *AdaptorSession) SwapAndRepay(tokenIn common.Address, tokenToRepay common.Address, amountIn *big.Int, exchange uint8, params []byte) (*types.Transaction, error) {
	return _Adaptor.Contract.SwapAndRepay(&_Adaptor.TransactOpts, tokenIn, tokenToRepay, amountIn, exchange, params)
}

// SwapAndRepay is a paid mutator transaction binding the contract method 0x642c2d8e.
//
// Solidity: function swapAndRepay(address tokenIn, address tokenToRepay, uint256 amountIn, uint8 exchange, bytes params) returns()
func (_Adaptor *AdaptorTransactorSession) SwapAndRepay(tokenIn common.Address, tokenToRepay common.Address, amountIn *big.Int, exchange uint8, params []byte) (*types.Transaction, error) {
	return _Adaptor.Contract.SwapAndRepay(&_Adaptor.TransactOpts, tokenIn, tokenToRepay, amountIn, exchange, params)
}

// AdaptorBorrowFromAaveIterator is returned from FilterBorrowFromAave and is used to iterate over the raw logs and unpacked data for BorrowFromAave events raised by the Adaptor contract.
type AdaptorBorrowFromAaveIterator struct {
	Event *AdaptorBorrowFromAave // Event containing the contract specifics and raw log

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
func (it *AdaptorBorrowFromAaveIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(AdaptorBorrowFromAave)
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
		it.Event = new(AdaptorBorrowFromAave)
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
func (it *AdaptorBorrowFromAaveIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *AdaptorBorrowFromAaveIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// AdaptorBorrowFromAave represents a BorrowFromAave event raised by the Adaptor contract.
type AdaptorBorrowFromAave struct {
	DebtTokenToBorrow common.Address
	AmountToBorrow    *big.Int
	Raw               types.Log // Blockchain specific contextual infos
}

// FilterBorrowFromAave is a free log retrieval operation binding the contract event 0xea4cf357bf825be590dc314053c8ab5f9306727eb7da5475212550cdf569acf7.
//
// Solidity: event BorrowFromAave(address debtTokenToBorrow, uint256 amountToBorrow)
func (_Adaptor *AdaptorFilterer) FilterBorrowFromAave(opts *bind.FilterOpts) (*AdaptorBorrowFromAaveIterator, error) {

	logs, sub, err := _Adaptor.contract.FilterLogs(opts, "BorrowFromAave")
	if err != nil {
		return nil, err
	}
	return &AdaptorBorrowFromAaveIterator{contract: _Adaptor.contract, event: "BorrowFromAave", logs: logs, sub: sub}, nil
}

// WatchBorrowFromAave is a free log subscription operation binding the contract event 0xea4cf357bf825be590dc314053c8ab5f9306727eb7da5475212550cdf569acf7.
//
// Solidity: event BorrowFromAave(address debtTokenToBorrow, uint256 amountToBorrow)
func (_Adaptor *AdaptorFilterer) WatchBorrowFromAave(opts *bind.WatchOpts, sink chan<- *AdaptorBorrowFromAave) (event.Subscription, error) {

	logs, sub, err := _Adaptor.contract.WatchLogs(opts, "BorrowFromAave")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(AdaptorBorrowFromAave)
				if err := _Adaptor.contract.UnpackLog(event, "BorrowFromAave", log); err != nil {
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

// ParseBorrowFromAave is a log parse operation binding the contract event 0xea4cf357bf825be590dc314053c8ab5f9306727eb7da5475212550cdf569acf7.
//
// Solidity: event BorrowFromAave(address debtTokenToBorrow, uint256 amountToBorrow)
func (_Adaptor *AdaptorFilterer) ParseBorrowFromAave(log types.Log) (*AdaptorBorrowFromAave, error) {
	event := new(AdaptorBorrowFromAave)
	if err := _Adaptor.contract.UnpackLog(event, "BorrowFromAave", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// AdaptorClaimCompAndSwapIterator is returned from FilterClaimCompAndSwap and is used to iterate over the raw logs and unpacked data for ClaimCompAndSwap events raised by the Adaptor contract.
type AdaptorClaimCompAndSwapIterator struct {
	Event *AdaptorClaimCompAndSwap // Event containing the contract specifics and raw log

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
func (it *AdaptorClaimCompAndSwapIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(AdaptorClaimCompAndSwap)
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
		it.Event = new(AdaptorClaimCompAndSwap)
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
func (it *AdaptorClaimCompAndSwapIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *AdaptorClaimCompAndSwapIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// AdaptorClaimCompAndSwap represents a ClaimCompAndSwap event raised by the Adaptor contract.
type AdaptorClaimCompAndSwap struct {
	AssetOut common.Address
	Exchange uint8
	Params   []byte
	Slippage uint64
	Raw      types.Log // Blockchain specific contextual infos
}

// FilterClaimCompAndSwap is a free log retrieval operation binding the contract event 0x31b9e5c539383a73e7c83d24416b5f1cbe6661628f779c66796f1905ed10f361.
//
// Solidity: event ClaimCompAndSwap(address assetOut, uint8 exchange, bytes params, uint64 slippage)
func (_Adaptor *AdaptorFilterer) FilterClaimCompAndSwap(opts *bind.FilterOpts) (*AdaptorClaimCompAndSwapIterator, error) {

	logs, sub, err := _Adaptor.contract.FilterLogs(opts, "ClaimCompAndSwap")
	if err != nil {
		return nil, err
	}
	return &AdaptorClaimCompAndSwapIterator{contract: _Adaptor.contract, event: "ClaimCompAndSwap", logs: logs, sub: sub}, nil
}

// WatchClaimCompAndSwap is a free log subscription operation binding the contract event 0x31b9e5c539383a73e7c83d24416b5f1cbe6661628f779c66796f1905ed10f361.
//
// Solidity: event ClaimCompAndSwap(address assetOut, uint8 exchange, bytes params, uint64 slippage)
func (_Adaptor *AdaptorFilterer) WatchClaimCompAndSwap(opts *bind.WatchOpts, sink chan<- *AdaptorClaimCompAndSwap) (event.Subscription, error) {

	logs, sub, err := _Adaptor.contract.WatchLogs(opts, "ClaimCompAndSwap")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(AdaptorClaimCompAndSwap)
				if err := _Adaptor.contract.UnpackLog(event, "ClaimCompAndSwap", log); err != nil {
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

// ParseClaimCompAndSwap is a log parse operation binding the contract event 0x31b9e5c539383a73e7c83d24416b5f1cbe6661628f779c66796f1905ed10f361.
//
// Solidity: event ClaimCompAndSwap(address assetOut, uint8 exchange, bytes params, uint64 slippage)
func (_Adaptor *AdaptorFilterer) ParseClaimCompAndSwap(log types.Log) (*AdaptorClaimCompAndSwap, error) {
	event := new(AdaptorClaimCompAndSwap)
	if err := _Adaptor.contract.UnpackLog(event, "ClaimCompAndSwap", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// AdaptorOwnerUpdatedIterator is returned from FilterOwnerUpdated and is used to iterate over the raw logs and unpacked data for OwnerUpdated events raised by the Adaptor contract.
type AdaptorOwnerUpdatedIterator struct {
	Event *AdaptorOwnerUpdated // Event containing the contract specifics and raw log

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
func (it *AdaptorOwnerUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(AdaptorOwnerUpdated)
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
		it.Event = new(AdaptorOwnerUpdated)
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
func (it *AdaptorOwnerUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *AdaptorOwnerUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// AdaptorOwnerUpdated represents a OwnerUpdated event raised by the Adaptor contract.
type AdaptorOwnerUpdated struct {
	User     common.Address
	NewOwner common.Address
	Raw      types.Log // Blockchain specific contextual infos
}

// FilterOwnerUpdated is a free log retrieval operation binding the contract event 0x8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d76.
//
// Solidity: event OwnerUpdated(address indexed user, address indexed newOwner)
func (_Adaptor *AdaptorFilterer) FilterOwnerUpdated(opts *bind.FilterOpts, user []common.Address, newOwner []common.Address) (*AdaptorOwnerUpdatedIterator, error) {

	var userRule []interface{}
	for _, userItem := range user {
		userRule = append(userRule, userItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _Adaptor.contract.FilterLogs(opts, "OwnerUpdated", userRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return &AdaptorOwnerUpdatedIterator{contract: _Adaptor.contract, event: "OwnerUpdated", logs: logs, sub: sub}, nil
}

// WatchOwnerUpdated is a free log subscription operation binding the contract event 0x8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d76.
//
// Solidity: event OwnerUpdated(address indexed user, address indexed newOwner)
func (_Adaptor *AdaptorFilterer) WatchOwnerUpdated(opts *bind.WatchOpts, sink chan<- *AdaptorOwnerUpdated, user []common.Address, newOwner []common.Address) (event.Subscription, error) {

	var userRule []interface{}
	for _, userItem := range user {
		userRule = append(userRule, userItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _Adaptor.contract.WatchLogs(opts, "OwnerUpdated", userRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(AdaptorOwnerUpdated)
				if err := _Adaptor.contract.UnpackLog(event, "OwnerUpdated", log); err != nil {
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

// ParseOwnerUpdated is a log parse operation binding the contract event 0x8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d76.
//
// Solidity: event OwnerUpdated(address indexed user, address indexed newOwner)
func (_Adaptor *AdaptorFilterer) ParseOwnerUpdated(log types.Log) (*AdaptorOwnerUpdated, error) {
	event := new(AdaptorOwnerUpdated)
	if err := _Adaptor.contract.UnpackLog(event, "OwnerUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// AdaptorSwapAndRepayIterator is returned from FilterSwapAndRepay and is used to iterate over the raw logs and unpacked data for SwapAndRepay events raised by the Adaptor contract.
type AdaptorSwapAndRepayIterator struct {
	Event *AdaptorSwapAndRepay // Event containing the contract specifics and raw log

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
func (it *AdaptorSwapAndRepayIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(AdaptorSwapAndRepay)
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
		it.Event = new(AdaptorSwapAndRepay)
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
func (it *AdaptorSwapAndRepayIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *AdaptorSwapAndRepayIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// AdaptorSwapAndRepay represents a SwapAndRepay event raised by the Adaptor contract.
type AdaptorSwapAndRepay struct {
	TokenIn      common.Address
	TokenToRepay common.Address
	AmountIn     *big.Int
	Exchange     uint8
	Params       []byte
	Raw          types.Log // Blockchain specific contextual infos
}

// FilterSwapAndRepay is a free log retrieval operation binding the contract event 0xb9d1059b0cfe1e79c30cc0b8a8a31f46b90f5536c468a92efa8e7ac584afb1d3.
//
// Solidity: event SwapAndRepay(address tokenIn, address tokenToRepay, uint256 amountIn, uint8 exchange, bytes params)
func (_Adaptor *AdaptorFilterer) FilterSwapAndRepay(opts *bind.FilterOpts) (*AdaptorSwapAndRepayIterator, error) {

	logs, sub, err := _Adaptor.contract.FilterLogs(opts, "SwapAndRepay")
	if err != nil {
		return nil, err
	}
	return &AdaptorSwapAndRepayIterator{contract: _Adaptor.contract, event: "SwapAndRepay", logs: logs, sub: sub}, nil
}

// WatchSwapAndRepay is a free log subscription operation binding the contract event 0xb9d1059b0cfe1e79c30cc0b8a8a31f46b90f5536c468a92efa8e7ac584afb1d3.
//
// Solidity: event SwapAndRepay(address tokenIn, address tokenToRepay, uint256 amountIn, uint8 exchange, bytes params)
func (_Adaptor *AdaptorFilterer) WatchSwapAndRepay(opts *bind.WatchOpts, sink chan<- *AdaptorSwapAndRepay) (event.Subscription, error) {

	logs, sub, err := _Adaptor.contract.WatchLogs(opts, "SwapAndRepay")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(AdaptorSwapAndRepay)
				if err := _Adaptor.contract.UnpackLog(event, "SwapAndRepay", log); err != nil {
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

// ParseSwapAndRepay is a log parse operation binding the contract event 0xb9d1059b0cfe1e79c30cc0b8a8a31f46b90f5536c468a92efa8e7ac584afb1d3.
//
// Solidity: event SwapAndRepay(address tokenIn, address tokenToRepay, uint256 amountIn, uint8 exchange, bytes params)
func (_Adaptor *AdaptorFilterer) ParseSwapAndRepay(log types.Log) (*AdaptorSwapAndRepay, error) {
	event := new(AdaptorSwapAndRepay)
	if err := _Adaptor.contract.UnpackLog(event, "SwapAndRepay", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}
