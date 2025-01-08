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

// CellarV25MetaData contains all meta data concerning the CellarV25 contract.
var CellarV25MetaData = &bind.MetaData{
	ABI: "[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"inputs\":[],\"name\":\"Cellar__InvalidFeeCut\",\"type\":\"error\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"OwnerUpdated\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint64\",\"name\":\"oldPlatformCut\",\"type\":\"uint64\"},{\"indexed\":false,\"internalType\":\"uint64\",\"name\":\"newPlatformCut\",\"type\":\"uint64\"}],\"name\":\"StrategistPlatformCutChanged\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"feeData\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"strategistPlatformCut\",\"type\":\"uint64\"},{\"internalType\":\"uint64\",\"name\":\"platformFee\",\"type\":\"uint64\"},{\"internalType\":\"uint64\",\"name\":\"lastAccrual\",\"type\":\"uint64\"},{\"internalType\":\"address\",\"name\":\"strategistPayoutAddress\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"setOwner\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"cut\",\"type\":\"uint64\"}],\"name\":\"setStrategistPlatformCut\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]",
}

// CellarV25ABI is the input ABI used to generate the binding from.
// Deprecated: Use CellarV25MetaData.ABI instead.
var CellarV25ABI = CellarV25MetaData.ABI

// CellarV25 is an auto generated Go binding around an Ethereum contract.
type CellarV25 struct {
	CellarV25Caller     // Read-only binding to the contract
	CellarV25Transactor // Write-only binding to the contract
	CellarV25Filterer   // Log filterer for contract events
}

// CellarV25Caller is an auto generated read-only Go binding around an Ethereum contract.
type CellarV25Caller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// CellarV25Transactor is an auto generated write-only Go binding around an Ethereum contract.
type CellarV25Transactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// CellarV25Filterer is an auto generated log filtering Go binding around an Ethereum contract events.
type CellarV25Filterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// CellarV25Session is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type CellarV25Session struct {
	Contract     *CellarV25        // Generic contract binding to set the session for
	CallOpts     bind.CallOpts     // Call options to use throughout this session
	TransactOpts bind.TransactOpts // Transaction auth options to use throughout this session
}

// CellarV25CallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type CellarV25CallerSession struct {
	Contract *CellarV25Caller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts    // Call options to use throughout this session
}

// CellarV25TransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type CellarV25TransactorSession struct {
	Contract     *CellarV25Transactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts    // Transaction auth options to use throughout this session
}

// CellarV25Raw is an auto generated low-level Go binding around an Ethereum contract.
type CellarV25Raw struct {
	Contract *CellarV25 // Generic contract binding to access the raw methods on
}

// CellarV25CallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type CellarV25CallerRaw struct {
	Contract *CellarV25Caller // Generic read-only contract binding to access the raw methods on
}

// CellarV25TransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type CellarV25TransactorRaw struct {
	Contract *CellarV25Transactor // Generic write-only contract binding to access the raw methods on
}

// NewCellarV25 creates a new instance of CellarV25, bound to a specific deployed contract.
func NewCellarV25(address common.Address, backend bind.ContractBackend) (*CellarV25, error) {
	contract, err := bindCellarV25(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &CellarV25{CellarV25Caller: CellarV25Caller{contract: contract}, CellarV25Transactor: CellarV25Transactor{contract: contract}, CellarV25Filterer: CellarV25Filterer{contract: contract}}, nil
}

// NewCellarV25Caller creates a new read-only instance of CellarV25, bound to a specific deployed contract.
func NewCellarV25Caller(address common.Address, caller bind.ContractCaller) (*CellarV25Caller, error) {
	contract, err := bindCellarV25(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &CellarV25Caller{contract: contract}, nil
}

// NewCellarV25Transactor creates a new write-only instance of CellarV25, bound to a specific deployed contract.
func NewCellarV25Transactor(address common.Address, transactor bind.ContractTransactor) (*CellarV25Transactor, error) {
	contract, err := bindCellarV25(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &CellarV25Transactor{contract: contract}, nil
}

// NewCellarV25Filterer creates a new log filterer instance of CellarV25, bound to a specific deployed contract.
func NewCellarV25Filterer(address common.Address, filterer bind.ContractFilterer) (*CellarV25Filterer, error) {
	contract, err := bindCellarV25(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &CellarV25Filterer{contract: contract}, nil
}

// bindCellarV25 binds a generic wrapper to an already deployed contract.
func bindCellarV25(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := abi.JSON(strings.NewReader(CellarV25ABI))
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_CellarV25 *CellarV25Raw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _CellarV25.Contract.CellarV25Caller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_CellarV25 *CellarV25Raw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _CellarV25.Contract.CellarV25Transactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_CellarV25 *CellarV25Raw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _CellarV25.Contract.CellarV25Transactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_CellarV25 *CellarV25CallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _CellarV25.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_CellarV25 *CellarV25TransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _CellarV25.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_CellarV25 *CellarV25TransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _CellarV25.Contract.contract.Transact(opts, method, params...)
}

// FeeData is a free data retrieval call binding the contract method 0xe753e600.
//
// Solidity: function feeData() view returns(uint64 strategistPlatformCut, uint64 platformFee, uint64 lastAccrual, address strategistPayoutAddress)
func (_CellarV25 *CellarV25Caller) FeeData(opts *bind.CallOpts) (struct {
	StrategistPlatformCut   uint64
	PlatformFee             uint64
	LastAccrual             uint64
	StrategistPayoutAddress common.Address
}, error) {
	var out []interface{}
	err := _CellarV25.contract.Call(opts, &out, "feeData")

	outstruct := new(struct {
		StrategistPlatformCut   uint64
		PlatformFee             uint64
		LastAccrual             uint64
		StrategistPayoutAddress common.Address
	})
	if err != nil {
		return *outstruct, err
	}

	outstruct.StrategistPlatformCut = *abi.ConvertType(out[0], new(uint64)).(*uint64)
	outstruct.PlatformFee = *abi.ConvertType(out[1], new(uint64)).(*uint64)
	outstruct.LastAccrual = *abi.ConvertType(out[2], new(uint64)).(*uint64)
	outstruct.StrategistPayoutAddress = *abi.ConvertType(out[3], new(common.Address)).(*common.Address)

	return *outstruct, err

}

// FeeData is a free data retrieval call binding the contract method 0xe753e600.
//
// Solidity: function feeData() view returns(uint64 strategistPlatformCut, uint64 platformFee, uint64 lastAccrual, address strategistPayoutAddress)
func (_CellarV25 *CellarV25Session) FeeData() (struct {
	StrategistPlatformCut   uint64
	PlatformFee             uint64
	LastAccrual             uint64
	StrategistPayoutAddress common.Address
}, error) {
	return _CellarV25.Contract.FeeData(&_CellarV25.CallOpts)
}

// FeeData is a free data retrieval call binding the contract method 0xe753e600.
//
// Solidity: function feeData() view returns(uint64 strategistPlatformCut, uint64 platformFee, uint64 lastAccrual, address strategistPayoutAddress)
func (_CellarV25 *CellarV25CallerSession) FeeData() (struct {
	StrategistPlatformCut   uint64
	PlatformFee             uint64
	LastAccrual             uint64
	StrategistPayoutAddress common.Address
}, error) {
	return _CellarV25.Contract.FeeData(&_CellarV25.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_CellarV25 *CellarV25Caller) Owner(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _CellarV25.contract.Call(opts, &out, "owner")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_CellarV25 *CellarV25Session) Owner() (common.Address, error) {
	return _CellarV25.Contract.Owner(&_CellarV25.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_CellarV25 *CellarV25CallerSession) Owner() (common.Address, error) {
	return _CellarV25.Contract.Owner(&_CellarV25.CallOpts)
}

// SetOwner is a paid mutator transaction binding the contract method 0x13af4035.
//
// Solidity: function setOwner(address newOwner) returns()
func (_CellarV25 *CellarV25Transactor) SetOwner(opts *bind.TransactOpts, newOwner common.Address) (*types.Transaction, error) {
	return _CellarV25.contract.Transact(opts, "setOwner", newOwner)
}

// SetOwner is a paid mutator transaction binding the contract method 0x13af4035.
//
// Solidity: function setOwner(address newOwner) returns()
func (_CellarV25 *CellarV25Session) SetOwner(newOwner common.Address) (*types.Transaction, error) {
	return _CellarV25.Contract.SetOwner(&_CellarV25.TransactOpts, newOwner)
}

// SetOwner is a paid mutator transaction binding the contract method 0x13af4035.
//
// Solidity: function setOwner(address newOwner) returns()
func (_CellarV25 *CellarV25TransactorSession) SetOwner(newOwner common.Address) (*types.Transaction, error) {
	return _CellarV25.Contract.SetOwner(&_CellarV25.TransactOpts, newOwner)
}

// SetStrategistPlatformCut is a paid mutator transaction binding the contract method 0xb5292a99.
//
// Solidity: function setStrategistPlatformCut(uint64 cut) returns()
func (_CellarV25 *CellarV25Transactor) SetStrategistPlatformCut(opts *bind.TransactOpts, cut uint64) (*types.Transaction, error) {
	return _CellarV25.contract.Transact(opts, "setStrategistPlatformCut", cut)
}

// SetStrategistPlatformCut is a paid mutator transaction binding the contract method 0xb5292a99.
//
// Solidity: function setStrategistPlatformCut(uint64 cut) returns()
func (_CellarV25 *CellarV25Session) SetStrategistPlatformCut(cut uint64) (*types.Transaction, error) {
	return _CellarV25.Contract.SetStrategistPlatformCut(&_CellarV25.TransactOpts, cut)
}

// SetStrategistPlatformCut is a paid mutator transaction binding the contract method 0xb5292a99.
//
// Solidity: function setStrategistPlatformCut(uint64 cut) returns()
func (_CellarV25 *CellarV25TransactorSession) SetStrategistPlatformCut(cut uint64) (*types.Transaction, error) {
	return _CellarV25.Contract.SetStrategistPlatformCut(&_CellarV25.TransactOpts, cut)
}

// CellarV25OwnerUpdatedIterator is returned from FilterOwnerUpdated and is used to iterate over the raw logs and unpacked data for OwnerUpdated events raised by the CellarV25 contract.
type CellarV25OwnerUpdatedIterator struct {
	Event *CellarV25OwnerUpdated // Event containing the contract specifics and raw log

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
func (it *CellarV25OwnerUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(CellarV25OwnerUpdated)
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
		it.Event = new(CellarV25OwnerUpdated)
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
func (it *CellarV25OwnerUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *CellarV25OwnerUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// CellarV25OwnerUpdated represents a OwnerUpdated event raised by the CellarV25 contract.
type CellarV25OwnerUpdated struct {
	User     common.Address
	NewOwner common.Address
	Raw      types.Log // Blockchain specific contextual infos
}

// FilterOwnerUpdated is a free log retrieval operation binding the contract event 0x8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d76.
//
// Solidity: event OwnerUpdated(address indexed user, address indexed newOwner)
func (_CellarV25 *CellarV25Filterer) FilterOwnerUpdated(opts *bind.FilterOpts, user []common.Address, newOwner []common.Address) (*CellarV25OwnerUpdatedIterator, error) {

	var userRule []interface{}
	for _, userItem := range user {
		userRule = append(userRule, userItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _CellarV25.contract.FilterLogs(opts, "OwnerUpdated", userRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return &CellarV25OwnerUpdatedIterator{contract: _CellarV25.contract, event: "OwnerUpdated", logs: logs, sub: sub}, nil
}

// WatchOwnerUpdated is a free log subscription operation binding the contract event 0x8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d76.
//
// Solidity: event OwnerUpdated(address indexed user, address indexed newOwner)
func (_CellarV25 *CellarV25Filterer) WatchOwnerUpdated(opts *bind.WatchOpts, sink chan<- *CellarV25OwnerUpdated, user []common.Address, newOwner []common.Address) (event.Subscription, error) {

	var userRule []interface{}
	for _, userItem := range user {
		userRule = append(userRule, userItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _CellarV25.contract.WatchLogs(opts, "OwnerUpdated", userRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(CellarV25OwnerUpdated)
				if err := _CellarV25.contract.UnpackLog(event, "OwnerUpdated", log); err != nil {
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
func (_CellarV25 *CellarV25Filterer) ParseOwnerUpdated(log types.Log) (*CellarV25OwnerUpdated, error) {
	event := new(CellarV25OwnerUpdated)
	if err := _CellarV25.contract.UnpackLog(event, "OwnerUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// CellarV25StrategistPlatformCutChangedIterator is returned from FilterStrategistPlatformCutChanged and is used to iterate over the raw logs and unpacked data for StrategistPlatformCutChanged events raised by the CellarV25 contract.
type CellarV25StrategistPlatformCutChangedIterator struct {
	Event *CellarV25StrategistPlatformCutChanged // Event containing the contract specifics and raw log

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
func (it *CellarV25StrategistPlatformCutChangedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(CellarV25StrategistPlatformCutChanged)
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
		it.Event = new(CellarV25StrategistPlatformCutChanged)
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
func (it *CellarV25StrategistPlatformCutChangedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *CellarV25StrategistPlatformCutChangedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// CellarV25StrategistPlatformCutChanged represents a StrategistPlatformCutChanged event raised by the CellarV25 contract.
type CellarV25StrategistPlatformCutChanged struct {
	OldPlatformCut uint64
	NewPlatformCut uint64
	Raw            types.Log // Blockchain specific contextual infos
}

// FilterStrategistPlatformCutChanged is a free log retrieval operation binding the contract event 0xb5cc994a260a85a42d6588668221571ae0a14f0a28f9e4817a5195262102c868.
//
// Solidity: event StrategistPlatformCutChanged(uint64 oldPlatformCut, uint64 newPlatformCut)
func (_CellarV25 *CellarV25Filterer) FilterStrategistPlatformCutChanged(opts *bind.FilterOpts) (*CellarV25StrategistPlatformCutChangedIterator, error) {

	logs, sub, err := _CellarV25.contract.FilterLogs(opts, "StrategistPlatformCutChanged")
	if err != nil {
		return nil, err
	}
	return &CellarV25StrategistPlatformCutChangedIterator{contract: _CellarV25.contract, event: "StrategistPlatformCutChanged", logs: logs, sub: sub}, nil
}

// WatchStrategistPlatformCutChanged is a free log subscription operation binding the contract event 0xb5cc994a260a85a42d6588668221571ae0a14f0a28f9e4817a5195262102c868.
//
// Solidity: event StrategistPlatformCutChanged(uint64 oldPlatformCut, uint64 newPlatformCut)
func (_CellarV25 *CellarV25Filterer) WatchStrategistPlatformCutChanged(opts *bind.WatchOpts, sink chan<- *CellarV25StrategistPlatformCutChanged) (event.Subscription, error) {

	logs, sub, err := _CellarV25.contract.WatchLogs(opts, "StrategistPlatformCutChanged")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(CellarV25StrategistPlatformCutChanged)
				if err := _CellarV25.contract.UnpackLog(event, "StrategistPlatformCutChanged", log); err != nil {
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

// ParseStrategistPlatformCutChanged is a log parse operation binding the contract event 0xb5cc994a260a85a42d6588668221571ae0a14f0a28f9e4817a5195262102c868.
//
// Solidity: event StrategistPlatformCutChanged(uint64 oldPlatformCut, uint64 newPlatformCut)
func (_CellarV25 *CellarV25Filterer) ParseStrategistPlatformCutChanged(log types.Log) (*CellarV25StrategistPlatformCutChanged, error) {
	event := new(CellarV25StrategistPlatformCutChanged)
	if err := _CellarV25.contract.UnpackLog(event, "StrategistPlatformCutChanged", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}
